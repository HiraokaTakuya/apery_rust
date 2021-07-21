use crate::book::*;
#[cfg(feature = "kppt")]
use crate::evaluate::kppt::*;
#[cfg(feature = "material")]
use crate::evaluate::material::*;
use crate::movegen::*;
use crate::movepick::*;
use crate::movetypes::*;
use crate::piecevalue::*;
use crate::position::*;
use crate::search::*;
use crate::timeman::*;
use crate::tt::*;
use crate::types::*;
use crate::usioption::*;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicPtr, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

struct Breadcrumb {
    thread: AtomicPtr<*mut Thread>,
    key: AtomicU64,
}

pub struct Breadcrumbs {
    value: Vec<Breadcrumb>,
}

impl Breadcrumbs {
    pub fn new() -> Self {
        let len = 1024;
        let mut value = Vec::with_capacity(len);
        for _ in 0..len {
            value.push(Breadcrumb {
                thread: AtomicPtr::new(std::ptr::null_mut()),
                key: AtomicU64::new(0),
            });
        }
        Self { value }
    }
    fn get_mut(&mut self, key: Key) -> &mut Breadcrumb {
        let mask = self.value.len() - 1;
        let index = key.0 as usize & mask;
        unsafe { self.value.get_unchecked_mut(index) }
    }
}

struct ThreadHolding {
    location: *mut Breadcrumb,
    other_thread: bool,
    owning: bool,
}

impl ThreadHolding {
    fn new(this_thread: &mut Thread, pos_key: Key, ply: i32) -> Self {
        unsafe {
            let location: *mut Breadcrumb = if ply < 8 {
                (*this_thread.breadcrumbs).get_mut(pos_key) as *mut Breadcrumb
            } else {
                std::ptr::null_mut()
            };
            let mut other_thread = false;
            let mut owning = false;
            if !location.is_null() {
                let tmp = (*location).thread.load(Ordering::Relaxed);
                if tmp.is_null() {
                    (*location).thread.store(&mut (this_thread as *mut Thread), Ordering::Relaxed);
                    (*location).key.store(pos_key.0, Ordering::Relaxed);
                    owning = true;
                } else if tmp != &mut (this_thread as *mut Thread) && (*location).key.load(Ordering::Relaxed) == pos_key.0 {
                    other_thread = true;
                }
            }
            Self {
                location,
                other_thread,
                owning,
            }
        }
    }
    fn marked(&self) -> bool {
        self.other_thread
    }
}

impl Drop for ThreadHolding {
    fn drop(&mut self) {
        if self.owning {
            unsafe {
                (*self.location).thread.store(std::ptr::null_mut(), Ordering::Relaxed);
            }
        }
    }
}

pub struct StatsType;
impl StatsType {
    pub const NUM: usize = 2;
}

pub struct InCheckType;
impl InCheckType {
    pub const NUM: usize = 2;
}

struct Thread {
    idx: usize,
    pv_idx: usize,
    tt_hit_average: u64,
    sel_depth: i32,
    null_move_pruning_min_ply: i32,
    null_move_pruning_color: Color,
    position: Position,
    root_moves: RootMoves,
    root_depth: Depth,
    completed_depth: Depth,
    counter_moves: CounterMoveHistory,
    main_history: ButterflyHistory,
    low_ply_history: LowPlyHistory,
    capture_history: CapturePieceToHistory,
    continuation_history: [[ContinuationHistory; StatsType::NUM]; InCheckType::NUM],
    limits: LimitsType, // Clone from ThreadPool for fast access.
    tt: *mut TranspositionTable,
    timeman: Arc<Mutex<TimeManagement>>, // shold I use pointer for speedup?
    #[cfg(feature = "kppt")]
    ehash: *mut EvalHash,
    breadcrumbs: *mut Breadcrumbs,
    reductions: *mut Reductions,
    usi_options: UsiOptions,
    best_move_changes: Arc<AtomicU64>,
    best_move_changess: Vec<Arc<AtomicU64>>,

    nodes: Arc<AtomicI64>,
    // following variables are shared one object that ThreadPool has.
    best_previous_score: Arc<Mutex<Value>>,
    iter_values: Arc<Mutex<[Value; 4]>>,
    increase_depth: Arc<AtomicBool>,
    // following variables are used only main thread.
    previous_time_reduction: f64,
    calls_count: i32,
    stop_on_ponderhit: Arc<AtomicBool>,
    ponder: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    hide_all_output: Arc<AtomicBool>,
    nodess: Vec<Arc<AtomicI64>>,
}

unsafe impl std::marker::Send for Thread {} // for Thread::tt and Thread::ehash

struct ThreadPoolBase {
    threads: Vec<Arc<Mutex<Thread>>>,
}

pub struct ThreadPool {
    thread_pool_base: Arc<Mutex<ThreadPoolBase>>,
    nodess: Vec<Arc<AtomicI64>>,
    pub book: Option<Book>,
    timeman: Arc<Mutex<TimeManagement>>,
    best_previous_score: Arc<Mutex<Value>>,
    iter_values: Arc<Mutex<[Value; 4]>>,
    best_move_changess: Vec<Arc<AtomicU64>>,
    stop_on_ponderhit: Arc<AtomicBool>,
    pub ponder: Arc<AtomicBool>,
    pub stop: Arc<AtomicBool>,
    increase_depth: Arc<AtomicBool>,
    pub hide_all_output: Arc<AtomicBool>,
    pub limits: LimitsType,
    pub last_best_root_move: Arc<Mutex<Option<RootMove>>>, // Not for usi engine. For debug or some tools.
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Thread {
    fn is_main(&self) -> bool {
        self.idx == 0
    }
    fn clear(&mut self) {
        self.calls_count = 0;
        self.counter_moves.fill(None);
        self.main_history.fill(0);
        self.low_ply_history.fill(0);
        self.capture_history.fill(0);

        self.continuation_history.iter_mut().for_each(|x| {
            x.iter_mut().for_each(|y| {
                y.fill(0);
                y.v[Piece::EMPTY.0 as usize][0].fill(COUNTER_MOVE_PRUNE_THRESHOLD - 1);
            })
        });
    }
    fn iterative_deepening_loop(&mut self) {
        let mut stack = [Stack::new(); MAX_PLY as usize + 10];
        let mut best_value = -Value::INFINITE;
        let mut last_best_move = None;
        let mut last_best_move_depth = Depth::ZERO; // not Option<Depth>
        let mut delta = -Value::INFINITE;
        let mut alpha = -Value::INFINITE;
        let mut beta = Value::INFINITE;
        let mut time_reduction = 1.0;
        let mut total_best_move_changes = 0.0f64;
        let mut last_info_time: Option<std::time::Instant> = None;
        let mut iter_index = 0;
        for item in stack.iter_mut().take(CURRENT_STACK_INDEX) {
            item.continuation_history = self.continuation_history[0][0].sentinel();
        }
        if self.is_main() {
            let best_previous_score = *self.best_previous_score.lock().unwrap();
            if best_previous_score == Value::INFINITE {
                for item in self.iter_values.lock().unwrap().iter_mut() {
                    *item = Value::ZERO;
                }
            } else {
                for item in self.iter_values.lock().unwrap().iter_mut() {
                    *item = best_previous_score;
                }
            }
        }

        self.low_ply_history.keep_data_from_previous_search();

        let multi_pv = std::cmp::min(self.usi_options.get_i64(UsiOptions::MULTI_PV) as usize, self.root_moves.len());
        self.tt_hit_average = TT_HIT_AVERAGE_WINDOW * TT_HIT_AVERAGE_RESOLUTION / 2;

        let mut search_again_counter = 0;

        evaluate_at_root(&self.position, &mut stack);
        while {
            self.root_depth += Depth::ONE_PLY;
            self.root_depth
        } < Depth::MAX
            && !self.stop.load(Ordering::Relaxed)
            && !(self.limits.depth.is_some() && self.is_main() && self.root_depth.0 > Depth(self.limits.depth.unwrap() as i32).0)
        {
            if self.idx > 0 {
                let i = (self.idx - 1) % 20;
                if ((self.root_depth.0 + SKIP_PHASE[i]) / SKIP_SIZE[i]) % 2 != 0 {
                    continue;
                }
            }

            if self.is_main() {
                total_best_move_changes /= 2.0;
            }

            for rm in self.root_moves.iter_mut() {
                rm.previous_score = rm.score;
            }

            self.pv_idx = 0;

            if !self.increase_depth.load(Ordering::Relaxed) {
                search_again_counter += 1;
            }
            while self.pv_idx < multi_pv && !self.stop.load(Ordering::Relaxed) {
                self.sel_depth = 0;
                if self.root_depth >= Depth(4) {
                    let previous_score = self.root_moves[self.pv_idx].previous_score;
                    delta = Value(19);
                    alpha = std::cmp::max(previous_score - delta, -Value::INFINITE);
                    beta = std::cmp::min(previous_score + delta, Value::INFINITE);
                }

                let mut failed_high_count = 0;
                loop {
                    let adjusted_depth = std::cmp::max(
                        Depth::ONE_PLY,
                        self.root_depth - Depth(failed_high_count + search_again_counter),
                    );
                    best_value = self.search::<Pv>(&mut stack, alpha, beta, adjusted_depth, false);
                    self.root_moves[self.pv_idx..].sort_by(|x, y| y.cmp(x));
                    if self.stop.load(Ordering::Relaxed) {
                        break;
                    }
                    if self.is_main()
                        && multi_pv == 1
                        && (best_value <= alpha || beta <= best_value)
                        && self.timeman.lock().unwrap().elapsed() > 3000
                        && (self.root_depth < Depth(10)
                            || last_info_time.is_none()
                            || last_info_time.unwrap().elapsed().as_millis() > 200)
                    {
                        last_info_time = Some(std::time::Instant::now());
                        if !self.hide_all_output.load(Ordering::Relaxed) {
                            println!(
                                "{}",
                                self.pv_info_to_usi_string(self.nodes_searched(), multi_pv, self.root_depth, alpha, beta, false,)
                            );
                        }
                    }
                    if best_value <= alpha {
                        beta = (alpha + beta) / 2;
                        alpha = std::cmp::max(best_value - delta, -Value::INFINITE);

                        failed_high_count = 0;
                        if self.is_main() {
                            self.stop_on_ponderhit.store(false, Ordering::Relaxed);
                        }
                    } else if beta <= best_value {
                        beta = std::cmp::min(best_value + delta, Value::INFINITE);
                        failed_high_count += 1;
                    } else {
                        self.root_moves[self.pv_idx].best_move_count += 1;
                        break;
                    }

                    delta += delta / 4 + Value(5);
                    debug_assert!(-Value::INFINITE <= alpha && beta <= Value::INFINITE);
                }

                self.root_moves[0..=self.pv_idx].sort_by(|x, y| y.cmp(x));

                if self.is_main()
                    && (self.stop.load(Ordering::Relaxed)
                        || self.pv_idx + 1 == multi_pv
                        || self.timeman.lock().unwrap().elapsed() > 3000)
                    && (self.root_depth < Depth(10)
                        || last_info_time.is_none()
                        || last_info_time.unwrap().elapsed().as_millis() > 200)
                {
                    last_info_time = Some(std::time::Instant::now());
                    if !self.hide_all_output.load(Ordering::Relaxed) {
                        println!(
                            "{}",
                            self.pv_info_to_usi_string(self.nodes_searched(), multi_pv, self.root_depth, alpha, beta, false,)
                        );
                    }
                }

                self.pv_idx += 1;
            }

            if !self.stop.load(Ordering::Relaxed) {
                self.completed_depth = self.root_depth;
            }

            if last_best_move.is_none() || last_best_move.non_zero_unwrap_unchecked() != self.root_moves[0].pv[0] {
                last_best_move = Some(self.root_moves[0].pv[0]);
                last_best_move_depth = self.root_depth;
            }

            if let Some(mate) = self.limits.mate {
                if best_value >= Value::MATE_IN_MAX_PLY && Value::MATE - best_value <= Value(mate as i32) {
                    self.stop.store(true, Ordering::Relaxed);
                }
            }

            if !self.is_main() {
                continue;
            }

            if self.limits.use_time_management()
                && !self.stop.load(Ordering::Relaxed)
                && !self.stop_on_ponderhit.load(Ordering::Relaxed)
            {
                let falling_eval = f64::from(
                    296 + 6 * (self.best_previous_score.lock().unwrap().0 - best_value.0)
                        + 6 * (self.iter_values.lock().unwrap()[iter_index].0 - best_value.0),
                ) / 725.0;
                let falling_eval = num::clamp(falling_eval, 0.5, 1.5);
                time_reduction = if last_best_move_depth.0 + 10 < self.completed_depth.0 {
                    1.92
                } else {
                    0.95
                };
                let reduction = (1.47 + self.previous_time_reduction) / (2.22 * time_reduction);
                for best_move_changes in self.best_move_changess.iter() {
                    total_best_move_changes += best_move_changes.load(Ordering::Relaxed) as f64;
                    best_move_changes.store(0, Ordering::Relaxed);
                }
                let best_move_instability = 1.0 + total_best_move_changes / self.best_move_changess.len() as f64;
                let (elapsed, optimum_millis) = {
                    let timeman = self.timeman.lock().unwrap();
                    (timeman.elapsed(), timeman.optimum_millis())
                };
                let total_time = if self.root_moves.len() == 1 {
                    0
                } else {
                    (optimum_millis as f64 * falling_eval * reduction * best_move_instability) as i64
                };
                if elapsed > total_time {
                    if self.ponder.load(Ordering::Relaxed) {
                        self.stop_on_ponderhit.store(true, Ordering::Relaxed);
                    } else {
                        self.stop.store(true, Ordering::Relaxed);
                    }
                } else if self.increase_depth.load(Ordering::Relaxed)
                    && self.ponder.load(Ordering::Relaxed)
                    && elapsed as f64 > total_time as f64 * 0.56
                {
                    self.increase_depth.store(false, Ordering::Relaxed);
                } else {
                    self.increase_depth.store(true, Ordering::Relaxed);
                }
            }
            self.iter_values.lock().unwrap()[iter_index] = best_value;
            iter_index = (iter_index + 1) & 3;
        }

        if !self.is_main() {
            return;
        }

        self.previous_time_reduction = time_reduction;
    }
    fn search<IsPv: Bool>(&mut self, stack: &mut [Stack], alpha: Value, beta: Value, depth: Depth, cut_node: bool) -> Value {
        let pv_node: bool = IsPv::BOOL;
        let root_node = pv_node && get_stack(stack, 0).ply == 0;

        if depth < Depth::ONE_PLY {
            return self.qsearch::<IsPv>(stack, alpha, beta, Depth::ZERO);
        }

        debug_assert!(-Value::INFINITE <= alpha && alpha < beta && beta <= Value::INFINITE);
        debug_assert!(pv_node || (alpha == beta - Value(1)));
        debug_assert!(Depth::ZERO < depth && depth < Depth::MAX);
        debug_assert!(!(pv_node && cut_node));

        // Step 1
        get_stack_mut(stack, 0).in_check = self.position.in_check();
        let prior_capture = self.position.captured_piece();
        let us = self.position.side_to_move();
        let mut best_value = -Value::INFINITE;
        let max_value = Value::INFINITE;

        if self.is_main() {
            self.check_time();
        }

        if pv_node && self.sel_depth < get_stack(stack, 0).ply + 1 {
            self.sel_depth = get_stack(stack, 0).ply + 1;
        }

        let mut alpha = alpha;
        let mut beta = beta;
        if !root_node {
            // Step 2
            match self.position.is_repetition() {
                Repetition::Not => {
                    if self.stop.load(Ordering::Relaxed) || get_stack(stack, 0).ply >= MAX_PLY {
                        return if get_stack(stack, 0).ply >= MAX_PLY && !get_stack(stack, 0).in_check {
                            evaluate(
                                &mut self.position,
                                stack,
                                #[cfg(feature = "kppt")]
                                self.ehash,
                            )
                        } else {
                            value_draw(self.nodes.load(Ordering::Relaxed))
                        };
                    }
                }
                Repetition::Draw => return Value::DRAW,
                Repetition::Win => return value_mate_in(get_stack(stack, 0).ply),
                Repetition::Lose => return value_mated_in(get_stack(stack, 0).ply),
                Repetition::Superior => {
                    if get_stack(stack, 0).ply != 2 {
                        return Value::MATE_IN_MAX_PLY;
                    }
                }
                Repetition::Inferior => {
                    if get_stack(stack, 0).ply != 2 {
                        return Value::MATED_IN_MAX_PLY;
                    }
                }
            }

            // Step 3
            alpha = std::cmp::max(value_mated_in(get_stack(stack, 0).ply), alpha);
            beta = std::cmp::min(value_mate_in(get_stack(stack, 0).ply + 1), beta);
            if alpha >= beta {
                return alpha;
            }
        }

        debug_assert!(0 <= get_stack(stack, 0).ply && get_stack(stack, 0).ply < MAX_PLY);

        get_stack_mut(stack, 1).ply = get_stack(stack, 0).ply + 1;
        let mut best_move: Option<Move> = None;
        get_stack_mut(stack, 1).excluded_move = None;
        get_stack_mut(stack, 2).killers[0] = None;
        get_stack_mut(stack, 2).killers[1] = None;

        // get_stack(stack, -1).current_move can be None. None => prev_sq: Square(0)
        let prev_sq = get_stack(stack, -1).current_move.non_zero_unwrap_unchecked().to(); // todo: Move::NULL

        if root_node {
            get_stack_mut(stack, 4).stat_score = 0;
        } else {
            get_stack_mut(stack, 2).stat_score = 0;
        }

        // Step 4
        let excluded_move = get_stack(stack, 0).excluded_move;
        let key = if let Some(excluded_move) = excluded_move {
            Key(self.position.key().0 ^ Key::make_key(u64::from(excluded_move.0.get())).0)
        } else {
            self.position.key()
        };
        let (mut tte, mut tt_hit) = unsafe { (*self.tt).probe(key) };
        let mut tt_value = if tt_hit {
            value_from_tt(tte.value(), get_stack(stack, 0).ply)
        } else {
            Value::NONE
        };
        let mut tt_move = if root_node {
            Some(self.root_moves[self.pv_idx].pv[0])
        } else if tt_hit {
            tte.mv(&self.position)
        } else {
            None
        };
        let tt_pv = pv_node || (tt_hit && tte.is_pv());

        if tt_pv
            && depth > Depth(12)
            && get_stack(stack, 0).ply - 1 < LowPlyHistory::MAX_LPH as i32
            && prior_capture == Piece::EMPTY
            && get_stack(stack, -1).current_move.is_normal_move()
        {
            self.low_ply_history.update(
                get_stack(stack, 0).ply - 1,
                get_stack(stack, -1).current_move.non_zero_unwrap_unchecked(),
                stat_bonus(depth - Depth(5)),
            );
        }

        self.tt_hit_average = (TT_HIT_AVERAGE_WINDOW - 1) * self.tt_hit_average / TT_HIT_AVERAGE_WINDOW
            + TT_HIT_AVERAGE_RESOLUTION * u64::from(tt_hit);

        if !pv_node
            && tt_hit
            && tte.depth() >= depth
            && tt_value != Value::NONE
            && if tt_value >= beta {
                tte.bound().include_lower()
            } else {
                tte.bound().include_upper()
            }
        {
            if let Some(tt_move) = tt_move {
                if tt_value >= beta {
                    if !tt_move.is_capture_or_pawn_promotion(&self.position) {
                        debug_assert!(self.position.pseudo_legal::<SearchingType>(tt_move));
                        // tt_move is guaranteed to be pseudo_legal.
                        // If tt_move isn't checked for pseudo_legal,
                        // tt_move can be promotion move for the piece that can't promotion.
                        // Then can be as follows.
                        //     tt_move.piece_moved_after_move().0 >= Piece::NUM
                        // It causes "index out of bounds" in update_continuation_histoies() in in update_quiet_stats().
                        self.update_quiet_stats(stack, tt_move, stat_bonus(depth), depth);
                    }

                    if get_stack(stack, -1).move_count <= 2
                        && !(prior_capture == Piece::EMPTY // prev is capture
                             || get_stack(stack, -1)
                                .current_move
                                .non_zero_unwrap_unchecked()
                                .is_pawn_promotion())
                    {
                        update_continuation_histories(
                            stack,
                            self.position.piece_on(prev_sq),
                            prev_sq,
                            -stat_bonus(depth + Depth::ONE_PLY),
                        );
                    }
                } else if !(tt_move.is_capture(&self.position)/*|| tt_move.is_pawn_promotion()*/) {
                    let penalty = -stat_bonus(depth);
                    self.main_history.update(us, tt_move, penalty);
                    update_continuation_histories(&mut stack[1..], tt_move.piece_moved_after_move(), tt_move.to(), penalty);
                }
            }
            return tt_value;
        }

        // Step 5
        if self.position.is_entering_king_win() {
            best_value = Value::mate_in(get_stack(stack, 0).ply);
            if tt_move.is_none() || tt_move.non_zero_unwrap_unchecked() != Move::WIN {
                get_stack_mut(stack, 0).static_eval = best_value; // is this necessary?
                tte.save(
                    key,
                    value_to_tt(best_value, get_stack(stack, 0).ply),
                    tt_pv,
                    Bound::EXACT,
                    depth,
                    Some(Move::WIN),
                    best_value,
                    unsafe { (*self.tt).generation() },
                );
            }
            return best_value;
        }

        if !root_node && !get_stack(stack, 0).in_check {
            if let Some(mate_move) = self.position.mate_move_in_1ply() {
                best_value = Value::mate_in(get_stack(stack, 0).ply);
                get_stack_mut(stack, 0).static_eval = best_value; // is this necessary?
                tte.save(
                    key,
                    value_to_tt(best_value, get_stack(stack, 0).ply),
                    tt_pv,
                    Bound::EXACT,
                    depth,
                    Some(mate_move),
                    best_value,
                    unsafe { (*self.tt).generation() },
                );
                return best_value;
            }
        }

        let pure_static_eval = if root_node {
            evaluate_at_root(&self.position, stack)
        } else {
            evaluate(
                &mut self.position,
                stack,
                #[cfg(feature = "kppt")]
                self.ehash,
            )
        };
        let improving;
        // Step 6
        if get_stack(stack, 0).in_check {
            get_stack_mut(stack, 0).static_eval = pure_static_eval;
            improving = false;
        } else {
            let mut eval;
            if tt_hit {
                eval = tte.eval();
                get_stack_mut(stack, 0).static_eval = eval;
                if eval == Value::NONE {
                    eval = pure_static_eval;
                    get_stack_mut(stack, 0).static_eval = eval;
                }
                if eval == Value::NONE {
                    eval = value_draw(self.nodes.load(Ordering::Relaxed));
                }
                if tt_value != Value::NONE
                    && if tt_value > eval {
                        tte.bound().include_lower()
                    } else {
                        tte.bound().include_upper()
                    }
                {
                    eval = tt_value;
                }
            } else {
                if get_stack(stack, -1).current_move.is_some() {
                    let bonus = -get_stack(stack, -1).stat_score / 512;
                    eval = pure_static_eval + Value(bonus);
                    get_stack_mut(stack, 0).static_eval = eval;
                } else {
                    eval = -get_stack(stack, -1).static_eval + Value(2 * TEMPO.0);
                    get_stack_mut(stack, 0).static_eval = eval;
                }
                tte.save(key, Value::NONE, tt_pv, Bound::BOUND_NONE, Depth::NONE, None, eval, unsafe {
                    (*self.tt).generation()
                });
            }

            // Step 7
            if !root_node && depth == Depth::ONE_PLY && eval <= alpha - RAZOR_MARGIN {
                return self.qsearch::<IsPv>(stack, alpha, beta, Depth::ZERO);
            }
            improving = if get_stack(stack, -2).static_eval == Value::NONE {
                get_stack(stack, 0).static_eval > get_stack(stack, -4).static_eval
                    || get_stack(stack, -4).static_eval == Value::NONE
            } else {
                get_stack(stack, 0).static_eval > get_stack(stack, -2).static_eval
            };

            // Step 8
            if !pv_node && depth.0 < 8 && eval - futility_margin(depth) >= beta && eval < Value::KNOWN_WIN {
                return eval;
            }

            // Step 9
            if !pv_node
                && get_stack(stack, -1).current_move.is_some()
                && get_stack(stack, -1).stat_score < 23824
                && eval >= beta
                && eval >= get_stack(stack, 0).static_eval
                && get_stack(stack, 0).static_eval.0
                    >= beta.0 - 28 * depth.0 - 28 * i32::from(improving) + 94 * i32::from(tt_pv) + 200
                && excluded_move.is_none()
                && (get_stack(stack, 0).ply >= self.null_move_pruning_min_ply || us != self.null_move_pruning_color)
            {
                debug_assert!(eval - beta >= Value(0));
                let r = Depth((737 + 77 * depth.0) / 246 + std::cmp::min((eval.0 - beta.0) / 192, 3));
                get_stack_mut(stack, 0).current_move = Some(Move::NULL);
                get_stack_mut(stack, 0).continuation_history = self.continuation_history[0][0].sentinel();

                self.position.do_null_move();
                #[cfg(feature = "kppt")]
                {
                    // key is wrong. but it's no problem.
                    get_stack_mut(stack, 1).static_eval_raw = get_stack(stack, 0).static_eval_raw;
                }
                let mut null_value = -self.search::<NonPv>(&mut stack[1..], -beta, -beta + Value(1), depth - r, !cut_node);
                self.position.undo_null_move();

                if null_value >= beta {
                    if null_value >= Value::MATE_IN_MAX_PLY {
                        null_value = beta;
                    }

                    if self.null_move_pruning_min_ply != 0 || (beta.0.abs() < Value::KNOWN_WIN.0 && depth.0 < 13) {
                        return null_value;
                    }

                    debug_assert!(self.null_move_pruning_min_ply == 0);
                    self.null_move_pruning_min_ply = get_stack(stack, 0).ply + 3 * (depth.0 - r.0) / 4;
                    self.null_move_pruning_color = us;

                    let v = self.search::<NonPv>(stack, beta - Value(1), beta, depth - r, false);

                    self.null_move_pruning_min_ply = 0;
                    if v >= beta {
                        return null_value;
                    }
                }
            }

            let prob_cut_beta = Value(beta.0 + 176 - 49 * i32::from(improving));

            // Step 10
            if !pv_node
                && depth.0 > 4
                && beta.0.abs() < Value::MATE_IN_MAX_PLY.0
                && !(tt_hit && tte.depth() >= depth - Depth(3) && tt_value != Value::NONE && tt_value < prob_cut_beta)
            {
                if tt_hit
                    && tte.depth() >= depth - Depth(3)
                    && tt_value != Value::NONE
                    && tt_value >= prob_cut_beta
                    && tt_move.is_some()
                    && tt_move
                        .non_zero_unwrap_unchecked()
                        .is_capture_or_pawn_promotion(&self.position)
                {
                    return prob_cut_beta;
                }

                let raised_beta = Value(beta.0 + 176 - 49 * i32::from(improving));
                debug_assert!(raised_beta < Value::INFINITE);
                let mut mp = MovePickerForProbCut::new(
                    &self.position,
                    tt_move,
                    raised_beta - get_stack(stack, 0).static_eval,
                    &self.capture_history,
                );
                let mut prob_cut_count = 0;
                while let Some(m) = mp.next_move(&self.position) {
                    if !(prob_cut_count < 2 + 2 * i32::from(cut_node)) {
                        break;
                    }
                    if m != excluded_move.non_zero_unwrap_unchecked() && self.position.legal(m) {
                        prob_cut_count += 1;
                        get_stack_mut(stack, 0).current_move = Some(m);
                        get_stack_mut(stack, 0).continuation_history = self.continuation_history
                            [usize::from(get_stack(stack, 0).in_check)][(prior_capture != Piece::EMPTY) as usize]
                            .get_mut(m.piece_moved_after_move(), m.to());
                        debug_assert!(depth.0 >= 5);

                        let gives_check = self.position.gives_check(m);
                        self.position.do_move(m, gives_check);
                        #[cfg(feature = "kppt")]
                        get_stack_mut(stack, 1).static_eval_raw.set_not_evaluated();
                        let mut value =
                            -self.qsearch::<NonPv>(&mut stack[1..], -prob_cut_beta, -prob_cut_beta + Value(1), Depth::ZERO);
                        if value >= prob_cut_beta {
                            value = -self.search::<NonPv>(
                                &mut stack[1..],
                                -prob_cut_beta,
                                -prob_cut_beta + Value(1),
                                Depth(depth.0 - 4),
                                !cut_node,
                            );
                        }
                        self.position.undo_move(m);

                        if value >= prob_cut_beta {
                            if !(tt_hit && tte.depth() >= depth - Depth(3) && tt_value != Value::NONE) {
                                tte.save(
                                    key,
                                    value_to_tt(value, get_stack(stack, 0).ply),
                                    tt_pv,
                                    Bound::LOWER,
                                    depth - Depth(3),
                                    Some(m),
                                    get_stack(stack, 0).static_eval,
                                    unsafe { (*self.tt).generation() },
                                );
                            }
                            return value;
                        }
                    }
                }
            }

            // Step 11
            if depth.0 >= 7 && tt_move.is_none() {
                self.search::<IsPv>(stack, alpha, beta, Depth(depth.0 - 7), cut_node);

                let (tte_new, tt_hit_new) = unsafe { (*self.tt).probe(key) };
                tte = tte_new;
                tt_hit = tt_hit_new;
                tt_value = if tt_hit {
                    value_from_tt(tte.value(), get_stack(stack, 0).ply)
                } else {
                    Value::NONE
                };
                tt_move = if tt_hit { tte.mv(&self.position) } else { None };
            }
        }

        let cont_hists = [
            get_stack(stack, -1).continuation_history as *const PieceToHistory,
            get_stack(stack, -2).continuation_history as *const PieceToHistory,
            std::ptr::null(),
            get_stack(stack, -4).continuation_history as *const PieceToHistory,
            std::ptr::null(),
            get_stack(stack, -6).continuation_history as *const PieceToHistory,
        ];

        let counter_move = self.counter_moves.get(prev_sq, self.position.piece_on(prev_sq));

        let mut mp = MovePickerForMainSearch::new(
            &self.position,
            tt_move,
            depth,
            &self.main_history,
            &self.low_ply_history,
            &self.capture_history,
            &cont_hists,
            counter_move,
            &get_stack(stack, 0).killers,
            get_stack(stack, 0).ply,
        );

        let mut value = best_value;
        let mut move_count_pruning = false;
        let tt_capture = tt_move.is_some()
            && tt_move
                .non_zero_unwrap_unchecked()
                .is_capture_or_pawn_promotion(&self.position);
        let mut singular_quiet_lmr = false;
        let former_pv = tt_pv && !pv_node;

        let th = ThreadHolding::new(self, key, get_stack(stack, 0).ply);

        // Step 12
        let mut move_count = 0;
        const CAPTURES_SEARCHED_NUM: usize = 32;
        const QUIETS_SEARCHED_NUM: usize = 64;
        let mut captures_searched = arrayvec::ArrayVec::<[_; CAPTURES_SEARCHED_NUM]>::new();
        let mut quiets_searched = arrayvec::ArrayVec::<[_; QUIETS_SEARCHED_NUM]>::new();
        while let Some(m) = mp.next_move(&self.position, move_count_pruning) {
            debug_assert!(Some(m).is_normal_move());

            if m == excluded_move.non_zero_unwrap_unchecked() {
                continue;
            }

            if root_node && !self.root_moves.iter().skip(self.pv_idx).any(|x| x.pv[0] == m) {
                continue;
            }

            if !root_node && !self.position.legal(m) {
                continue;
            }

            move_count += 1;
            get_stack_mut(stack, 0).move_count = move_count;

            let mut extension = Depth::ZERO;
            let is_capture_or_pawn_promotion = m.is_capture_or_pawn_promotion(&self.position);
            let piece_moved_after_move = m.piece_moved_after_move();
            let gives_check = self.position.gives_check(m);

            let new_depth = depth - Depth::ONE_PLY;
            let to = m.to();

            // Step 13
            if !root_node && best_value > Value::MATED_IN_MAX_PLY {
                move_count_pruning = move_count >= futility_move_count(improving, depth.0);
                let lmr_depth = std::cmp::max(
                    new_depth - unsafe { (*self.reductions).get(improving, depth, move_count) },
                    Depth::ZERO,
                );
                if !is_capture_or_pawn_promotion && !gives_check {
                    if lmr_depth.0 < 3 + i32::from(get_stack(stack, -1).stat_score > 0 || get_stack(stack, -1).move_count == 1)
                        && unsafe { (*cont_hists[0]).get(to, piece_moved_after_move) } < i32::from(COUNTER_MOVE_PRUNE_THRESHOLD)
                        && unsafe { (*cont_hists[1]).get(to, piece_moved_after_move) } < i32::from(COUNTER_MOVE_PRUNE_THRESHOLD)
                    {
                        continue;
                    }
                    if lmr_depth < Depth(8)
                        && !get_stack(stack, 0).in_check
                        && get_stack(stack, 0).static_eval.0 + 284 + 188 * lmr_depth.0 <= alpha.0
                        && unsafe { (*cont_hists[0]).get(to, piece_moved_after_move) }
                            + unsafe { (*cont_hists[1]).get(to, piece_moved_after_move) }
                            + unsafe { (*cont_hists[3]).get(to, piece_moved_after_move) }
                            + unsafe { (*cont_hists[5]).get(to, piece_moved_after_move) } / 2
                            < 28388
                    {
                        continue;
                    }
                    if !self
                        .position
                        .see_ge(m, Value(-(29 - std::cmp::min(lmr_depth.0, 17)) * lmr_depth.0 * lmr_depth.0))
                    {
                        continue;
                    }
                } else {
                    if !gives_check
                        && lmr_depth < Depth::ONE_PLY
                        && self
                            .capture_history
                            .get(piece_moved_after_move, to, PieceType::new(self.position.piece_on(to)))
                            < 0
                    {
                        continue;
                    }
                    if !gives_check
                        && lmr_depth < Depth(6)
                        && !(pv_node && best_value.0.abs() < 2)
                        && piece_value(piece_moved_after_move) >= piece_value(self.position.piece_on(to))
                        && !get_stack(stack, 0).in_check
                        && Value(
                            get_stack(stack, 0).static_eval.0
                                + 178
                                + 261 * lmr_depth.0
                                + capture_piece_value(self.position.piece_on(to)).0,
                        ) <= alpha
                    {
                        continue;
                    }

                    if !self.position.see_ge(m, Value(-202 * depth.0)) {
                        continue;
                    }
                }
            }

            // Step 14
            if depth.0 >= 6
                && m == tt_move.non_zero_unwrap_unchecked()
                && !root_node
                && excluded_move.is_none()
                && tt_value.0.abs() < Value::KNOWN_WIN.0
                && tte.bound().include_lower()
                && tte.depth().0 >= depth.0 - 3
            {
                let singular_beta = Value(tt_value.0 - ((i32::from(former_pv) + 4) * depth.0) / 2);
                let singular_depth = Depth((depth.0 - 1 + 3 * i32::from(former_pv)) / 2);
                get_stack_mut(stack, 0).excluded_move = Some(m);
                value = self.search::<NonPv>(stack, singular_beta - Value(1), singular_beta, singular_depth, cut_node);
                get_stack_mut(stack, 0).excluded_move = None;
                if value < singular_beta {
                    extension = Depth::ONE_PLY;
                    singular_quiet_lmr = !tt_capture;
                } else if singular_beta >= beta {
                    return singular_beta;
                } else if tt_value >= beta {
                    get_stack_mut(stack, 0).excluded_move = Some(m);
                    value = self.search::<NonPv>(stack, beta - Value(1), beta, Depth((depth.0 + 3) / 2), cut_node);
                    get_stack_mut(stack, 0).excluded_move = None;

                    if value >= beta {
                        return beta;
                    }
                }
            } else if gives_check
                && ((!m.is_drop() && self.position.blockers_for_king(us.inverse()).is_set(m.from()))
                    || self.position.see_ge(m, Value::ZERO))
            {
                extension = Depth::ONE_PLY;
            }

            let new_depth = new_depth + extension;

            get_stack_mut(stack, 0).current_move = Some(m);
            get_stack_mut(stack, 0).continuation_history = self.continuation_history[usize::from(get_stack(stack, 0).in_check)]
                [(prior_capture != Piece::EMPTY) as usize]
                .get_mut(piece_moved_after_move, to);

            // Step 15
            self.position.do_move(m, gives_check);
            #[cfg(feature = "kppt")]
            get_stack_mut(stack, 1).static_eval_raw.set_not_evaluated();

            // Step 16
            let (do_full_depth_search, did_lmr) = if depth.0 >= 3
                && move_count > 1 + if root_node { 2 } else { 0 }
                && (!root_node || self.best_move_count(m) == 0)
                && (!is_capture_or_pawn_promotion
                    || move_count_pruning
                    || get_stack(stack, 0).static_eval + capture_piece_value(self.position.captured_piece()) <= alpha
                    || cut_node
                    || self.tt_hit_average < 415 * TT_HIT_AVERAGE_RESOLUTION * TT_HIT_AVERAGE_WINDOW / 1024)
            {
                let mut r = unsafe { (*self.reductions).get(improving, depth, move_count) };

                if cut_node && depth <= Depth(10) && move_count <= 2 && !get_stack(stack, 0).in_check {
                    r -= Depth::ONE_PLY;
                }

                if self.tt_hit_average > 473 * TT_HIT_AVERAGE_RESOLUTION * TT_HIT_AVERAGE_WINDOW / 1024 {
                    r -= Depth::ONE_PLY;
                }

                if th.marked() {
                    r += Depth::ONE_PLY;
                }

                //if tt_pv {
                //    r -= Depth(2 * Depth::ONE_PLY.0);
                //}

                if move_count_pruning && !former_pv {
                    r += Depth::ONE_PLY;
                }

                //if get_stack(stack, -1).move_count > 13 {
                //    r -= Depth::ONE_PLY;
                //}

                if singular_quiet_lmr {
                    r -= Depth(1 + i32::from(former_pv));
                }

                if !is_capture_or_pawn_promotion {
                    if tt_capture {
                        r += Depth::ONE_PLY;
                    }
                    if cut_node {
                        r += Depth(2);
                    } else if !self.position.see_ge(m.reverse(), Value::ZERO) {
                        r -= Depth(2 + i32::from(tt_pv) - i32::from(PieceType::new(piece_moved_after_move) == PieceType::PAWN));
                    }

                    get_stack_mut(stack, 0).stat_score = self.main_history.get(us, m)
                        + unsafe { (*cont_hists[0]).get(to, piece_moved_after_move) }
                        + unsafe { (*cont_hists[1]).get(to, piece_moved_after_move) }
                        + unsafe { (*cont_hists[3]).get(to, piece_moved_after_move) }
                        - 4826;

                    if get_stack(stack, 0).stat_score >= -100 && get_stack(stack, -1).stat_score < -112 {
                        r -= Depth::ONE_PLY;
                    } else if get_stack(stack, -1).stat_score >= -125 && get_stack(stack, 0).stat_score < -138 {
                        r += Depth::ONE_PLY;
                    }
                    r -= Depth(get_stack(stack, 0).stat_score / 14615);
                } else {
                    if depth < Depth(8) && move_count > 2 {
                        r += Depth::ONE_PLY;
                    }
                    if !gives_check
                        && Value(
                            get_stack(stack, 0).static_eval.0
                                + capture_piece_value(self.position.captured_piece()).0
                                + 211 * depth.0,
                        ) <= alpha
                    {
                        r += Depth::ONE_PLY;
                    }
                }
                //else if depth < Depth(8) && move_count > 2 {
                //    r += Depth::ONE_PLY;
                //}
                let d = std::cmp::max(new_depth - std::cmp::max(r, Depth::ZERO), Depth::ONE_PLY);
                value = -self.search::<NonPv>(&mut stack[1..], -(alpha + Value(1)), -alpha, d, true);
                (value > alpha && d != new_depth, true)
            } else {
                (!pv_node || move_count > 1, false)
            };

            // Step 17
            if do_full_depth_search {
                value = -self.search::<NonPv>(&mut stack[1..], -(alpha + Value(1)), -alpha, new_depth, !cut_node);

                if did_lmr && !is_capture_or_pawn_promotion {
                    let mut bonus = if value > alpha {
                        stat_bonus(new_depth)
                    } else {
                        -stat_bonus(new_depth)
                    };
                    if Some(m) == get_stack(stack, 0).killers[0] {
                        bonus += bonus / 4;
                    }
                    update_continuation_histories(stack, piece_moved_after_move, to, bonus);
                }
            }
            if pv_node && (move_count == 1 || (value > alpha && (root_node || value < beta))) {
                value = -self.search::<Pv>(&mut stack[1..], -beta, -alpha, new_depth, false);
            }

            // Step 18
            self.position.undo_move(m);

            debug_assert!(-Value::INFINITE < value && value < Value::INFINITE);

            // Step 19
            if self.stop.load(Ordering::Relaxed) {
                return Value::ZERO;
            }

            if root_node {
                let rm: &mut RootMove = self.root_moves.iter_mut().find(|x| x.pv[0] == m).unwrap();
                if move_count == 1 || value > alpha {
                    rm.score = value;
                    rm.sel_depth = self.sel_depth;
                    rm.pv.truncate(1);
                    rm.extract_pv_from_tt(&mut self.position, self.tt);
                    if move_count > 1 {
                        self.best_move_changes.fetch_add(1, Ordering::Relaxed);
                    }
                } else {
                    rm.score = -Value::INFINITE;
                }
            }

            if value > best_value {
                best_value = value;
                if value > alpha {
                    best_move = Some(m);
                    if pv_node && !root_node {
                        // todo: update_pv
                    }
                    if pv_node && value < beta {
                        alpha = value;
                    } else {
                        debug_assert!(value >= beta); // fail high
                        get_stack_mut(stack, 0).stat_score = 0;
                        break;
                    }
                }
            }

            if m != best_move.non_zero_unwrap_unchecked() {
                if is_capture_or_pawn_promotion {
                    let _ = captures_searched.try_push(m);
                } else if !is_capture_or_pawn_promotion {
                    let _ = quiets_searched.try_push(m);
                }
            }
        }

        fn legal_moves_size(pos: &Position) -> usize {
            let mut mlist = MoveList::new();
            let current_size = 0;
            mlist.generate::<LegalType>(pos, current_size);
            mlist.size
        }
        debug_assert!(
            move_count != 0 || !get_stack(stack, 0).in_check || excluded_move.is_some() || legal_moves_size(&self.position) == 0
        );

        if move_count == 0 {
            best_value = if excluded_move.is_some() {
                alpha
            } else {
                value_mated_in(get_stack(stack, 0).ply)
            };
        } else if let Some(best_move) = best_move {
            self.update_all_stats(
                stack,
                best_move,
                best_value,
                beta,
                prev_sq,
                &quiets_searched[..],
                &captures_searched[..],
                depth,
            );
        } else if (pv_node || depth.0 >= 3) && prior_capture == Piece::EMPTY {
            update_continuation_histories(stack, self.position.piece_on(prev_sq), prev_sq, stat_bonus(depth));
        }

        if pv_node {
            best_value = std::cmp::min(best_value, max_value);
        }

        if excluded_move.is_none() && !(root_node && self.pv_idx != 0) {
            tte.save(
                key,
                value_to_tt(best_value, get_stack(stack, 0).ply),
                tt_pv,
                if best_value >= beta {
                    Bound::LOWER
                } else if pv_node && best_move.is_some() {
                    Bound::EXACT
                } else {
                    Bound::UPPER
                },
                depth,
                best_move,
                get_stack(stack, 0).static_eval,
                unsafe { (*self.tt).generation() },
            );
        }

        debug_assert!(-Value::INFINITE < best_value && best_value < Value::INFINITE);

        best_value
    }
    fn qsearch<IsPv: Bool>(&mut self, stack: &mut [Stack], alpha: Value, beta: Value, depth: Depth) -> Value {
        let pv_node: bool = IsPv::BOOL;
        let mut alpha = alpha;

        let old_alpha = alpha;
        get_stack_mut(stack, 1).ply = get_stack(stack, 0).ply + 1;
        get_stack_mut(stack, 0).current_move = None;
        get_stack_mut(stack, 0).continuation_history = self.continuation_history[0][0].sentinel();
        let mut best_move: Option<Move> = None;
        get_stack_mut(stack, 0).in_check = self.position.in_check();
        let prior_capture = self.position.captured_piece();
        let mut _move_count = 0;

        // We don't have to check repetition.
        // Because qsearch use only capture-moves, promotion-moves, and evasion-moves.
        // Their moves don't reach repetition positions.
        if get_stack_mut(stack, 0).ply >= MAX_PLY {
            return Value::DRAW;
        }

        debug_assert!(0 <= get_stack(stack, 0).ply && get_stack(stack, 0).ply < MAX_PLY);

        let tt_depth = if get_stack(stack, 0).in_check || depth >= Depth::QS_CHECKS {
            Depth::QS_CHECKS
        } else {
            Depth::QS_NO_CHECKS
        };
        let key = self.position.key();
        let (tte, tt_hit) = unsafe { (*self.tt).probe(key) };
        let tt_value = if tt_hit {
            value_from_tt(tte.value(), get_stack(stack, 0).ply)
        } else {
            Value::NONE
        };
        let tt_move = if tt_hit { tte.mv(&self.position) } else { None };
        let pv_hit = tt_hit && tte.is_pv();

        if !pv_node
            && tt_hit
            && tte.depth() >= tt_depth
            && tt_value != Value::NONE // Only in case of TT access race
            && if tt_value >= beta {
                tte.bound().include_lower()
            } else {
                tte.bound().include_upper()
            }
        {
            return tt_value;
        }

        let mut best_value;
        let futility_base;
        if get_stack(stack, 0).in_check {
            get_stack_mut(stack, 0).static_eval = Value::NONE;
            futility_base = -Value::INFINITE;
            best_value = -Value::INFINITE;
        } else {
            if let Some(_mate_move) = self.position.mate_move_in_1ply() {
                return value_mate_in(get_stack(stack, 0).ply);
            }
            if tt_hit {
                best_value = tte.eval();
                get_stack_mut(stack, 0).static_eval = best_value;
                if best_value == Value::NONE {
                    best_value = evaluate(
                        &mut self.position,
                        stack,
                        #[cfg(feature = "kppt")]
                        self.ehash,
                    );
                    get_stack_mut(stack, 0).static_eval = best_value;
                }
                if tt_value != Value::NONE
                    && if tt_value > best_value {
                        tte.bound().include_lower()
                    } else {
                        tte.bound().include_upper()
                    }
                {
                    best_value = tt_value;
                }
            } else {
                best_value = if get_stack(stack, -1).current_move.non_zero_unwrap_unchecked() != Move::NULL {
                    evaluate(
                        &mut self.position,
                        stack,
                        #[cfg(feature = "kppt")]
                        self.ehash,
                    )
                } else {
                    -get_stack(stack, -1).static_eval + Value(2 * TEMPO.0)
                };
                get_stack_mut(stack, 0).static_eval = best_value;
            }

            if best_value >= beta {
                if !tt_hit {
                    tte.save(
                        key,
                        value_to_tt(best_value, get_stack(stack, 0).ply),
                        false,
                        Bound::LOWER,
                        Depth::NONE,
                        None,
                        get_stack(stack, 0).static_eval,
                        unsafe { (*self.tt).generation() },
                    );
                }
                return best_value;
            }

            if pv_node && best_value > alpha {
                alpha = best_value;
            }

            futility_base = best_value + Value(141);
        }

        let cont_hists = [
            get_stack(stack, -1).continuation_history as *const PieceToHistory,
            get_stack(stack, -2).continuation_history as *const PieceToHistory,
            std::ptr::null(),
            get_stack(stack, -4).continuation_history as *const PieceToHistory,
            std::ptr::null(),
            get_stack(stack, -6).continuation_history as *const PieceToHistory,
        ];
        let mut mp = MovePickerForQSearch::new(
            &self.main_history,
            &self.capture_history,
            &cont_hists,
            &self.position,
            get_stack(stack, -1).current_move.non_zero_unwrap_unchecked().to(),
            tt_move,
            depth,
        );

        evaluate(
            &mut self.position,
            stack,
            #[cfg(feature = "kppt")]
            self.ehash,
        ); // for difference calculation
        while let Some(m) = mp.next_move(&self.position) {
            debug_assert!(m != Move::NULL);
            let gives_check = self.position.gives_check(m);
            _move_count += 1;
            if !get_stack(stack, 0).in_check && !gives_check && futility_base > -Value::KNOWN_WIN {
                let futility_value = futility_base
                    + capture_piece_value(self.position.piece_on(m.to()))
                    + if m.is_promotion() {
                        promote_piece_type_value(PieceType::new(m.piece_moved_before_move()))
                    } else {
                        Value::ZERO
                    };

                if futility_value <= alpha {
                    best_value = std::cmp::max(best_value, futility_value);
                    continue;
                }

                if futility_base <= alpha && !self.position.see_ge(m, Value(1)) {
                    best_value = std::cmp::max(best_value, futility_base);
                    continue;
                }
            }

            if !get_stack(stack, 0).in_check && !self.position.see_ge(m, Value::ZERO) {
                continue;
            }

            if !self.position.legal(m) {
                _move_count -= 1;
                continue;
            }

            get_stack_mut(stack, 0).current_move = Some(m);
            get_stack_mut(stack, 0).continuation_history = self.continuation_history[usize::from(get_stack(stack, 0).in_check)]
                [(prior_capture != Piece::EMPTY) as usize]
                .get_mut(m.piece_moved_after_move(), m.to());

            self.position.do_move(m, gives_check);
            #[cfg(feature = "kppt")]
            get_stack_mut(stack, 1).static_eval_raw.set_not_evaluated();
            let value = -self.qsearch::<IsPv>(&mut stack[1..], -beta, -alpha, depth - Depth::ONE_PLY);
            self.position.undo_move(m);

            debug_assert!(-Value::INFINITE < value && value < Value::INFINITE);

            if value > best_value {
                best_value = value;

                if value > alpha {
                    best_move = Some(m);
                    if pv_node {
                        // todo: update_pv
                    }

                    if pv_node && value < beta {
                        alpha = value;
                    } else {
                        break; // fail high
                    }
                }
            }
        }

        if get_stack(stack, 0).in_check && best_value == -Value::INFINITE {
            return Value::mated_in(get_stack(stack, 0).ply);
        }

        tte.save(
            key,
            value_to_tt(best_value, get_stack(stack, 0).ply),
            pv_hit,
            if best_value >= beta {
                Bound::LOWER
            } else if pv_node && best_value > old_alpha {
                Bound::EXACT
            } else {
                Bound::UPPER
            },
            tt_depth,
            best_move,
            get_stack(stack, 0).static_eval,
            unsafe { (*self.tt).generation() },
        );

        debug_assert!(-Value::INFINITE < best_value && best_value < Value::INFINITE);

        best_value
    }
    fn best_move_count(&self, m: Move) -> usize {
        let rm = self.root_moves.iter().skip(self.pv_idx).find(|rm| rm.pv[0] == m);
        match rm {
            Some(rm) => rm.best_move_count,
            None => 0,
        }
    }
    fn nodes_searched(&self) -> i64 {
        debug_assert!(self.is_main());
        self.nodess.iter().fold(0, |sum, nodes| sum + nodes.load(Ordering::Relaxed))
    }
    fn check_time(&mut self) {
        self.calls_count -= 1;
        if self.calls_count > 0 {
            return;
        }
        self.calls_count = match self.limits.nodes {
            Some(nodes) => std::cmp::min(1024, nodes / 1024) as i32,
            None => 1024,
        };

        if self.ponder.load(Ordering::Relaxed) {
            return;
        }

        let elapsed = self.limits.start_time.unwrap().elapsed();

        if (self.limits.use_time_management()
            && (elapsed.as_millis() as i64 > self.timeman.lock().unwrap().maximum_millis() - 10
                || self.stop_on_ponderhit.load(Ordering::Relaxed)))
            || (self.limits.movetime.is_some() && elapsed >= self.limits.movetime.unwrap())
            || (self.limits.nodes.is_some() && self.nodes_searched() >= self.limits.nodes.unwrap() as i64)
        {
            self.stop.store(true, Ordering::Relaxed);
        }
    }
    fn update_all_stats(
        &mut self,
        stack: &mut [Stack],
        best_move: Move,
        best_value: Value,
        beta: Value,
        prev_sq: Square,
        quiets_searched: &[Move],
        captures_searched: &[Move],
        depth: Depth,
    ) {
        let us = self.position.side_to_move();
        let moved_piece = best_move.piece_moved_after_move();
        let captured = PieceType::new(self.position.piece_on(best_move.to()));
        let bonus1 = stat_bonus(depth + Depth::ONE_PLY);
        let bonus2 = if best_value > beta + piece_type_value(PieceType::PAWN) {
            bonus1
        } else {
            stat_bonus(depth)
        };
        if !best_move.is_capture_or_pawn_promotion(&self.position) {
            self.update_quiet_stats(stack, best_move, bonus2, depth);
            for &quiet_move in quiets_searched {
                self.main_history.update(us, quiet_move, -bonus2);
                update_continuation_histories(&mut stack[1..], quiet_move.piece_moved_after_move(), quiet_move.to(), -bonus2);
            }
        } else {
            let capture_history = &mut self.capture_history;
            capture_history.update(moved_piece, best_move.to(), captured, bonus1);
        }

        if (get_stack(stack, -1).move_count == 1 || get_stack(stack, -1).current_move == get_stack(stack, -1).killers[0])
            && self.position.captured_piece() == Piece::EMPTY
        {
            update_continuation_histories(stack, self.position.piece_on(prev_sq), prev_sq, -bonus1);
        }

        for &capture_move in captures_searched {
            let moved_piece = capture_move.piece_moved_after_move();
            let captured = PieceType::new(self.position.piece_on(capture_move.to()));
            self.capture_history.update(moved_piece, capture_move.to(), captured, -bonus1);
        }
    }
    fn update_quiet_stats(&mut self, stack: &mut [Stack], m: Move, bonus: i32, depth: Depth) {
        if get_stack(stack, 0).killers[0].non_zero_unwrap_unchecked() != m {
            let ss = get_stack_mut(stack, 0);
            ss.killers[1] = ss.killers[0];
            ss.killers[0] = Some(m);
        }
        let us = self.position.side_to_move();
        self.main_history.update(us, m, bonus);
        update_continuation_histories(&mut stack[1..], m.piece_moved_after_move(), m.to(), bonus);

        if PieceType::new(m.piece_moved_after_move()) == PieceType::PAWN {
            self.main_history.update(us, m, -bonus);
        }

        let prev_move = get_stack(stack, -1).current_move;
        if prev_move.is_normal_move() {
            let prev_sq = prev_move.non_zero_unwrap_unchecked().to();
            self.counter_moves.set(prev_sq, self.position.piece_on(prev_sq), m);
        }
        if depth.0 > 11 && get_stack(stack, 0).ply < LowPlyHistory::MAX_LPH as i32 {
            self.low_ply_history
                .update(get_stack(stack, 0).ply, m, stat_bonus(depth - Depth(7)));
        }
    }
    fn pv_info_to_usi_string(
        &self,
        nodes_searched: i64,
        multi_pv: usize,
        depth: Depth,
        alpha: Value,
        beta: Value,
        reverse: bool, // for Shogidokoro Graph
    ) -> String {
        let elapsed_millis = self.limits.start_time.unwrap().elapsed().as_millis() as i64 + 1; // "+ 1": avoid dividing by 0
        let info_with_multi_pv_index = |i: usize, rm: &RootMove| -> Option<String> {
            let updated = rm.score != -Value::INFINITE;
            if depth == Depth::ONE_PLY && !updated {
                return None;
            }
            let (d, v) = if updated {
                (depth, rm.score)
            } else {
                (depth - Depth::ONE_PLY, rm.previous_score)
            };
            let line = format!(
                "info depth {depth} seldepth {seldepth} multipv {multipv} score {score} {bound}nodes {nodes} nps {nps} time {time} pv {pv}",
                depth = d.0,
                seldepth = rm.sel_depth,
                multipv = i + 1,
                score = v.to_usi(),
                bound = if v >= beta {
                    "lowerbound "
                } else if v <= alpha {
                    "upperbound "
                } else {""},
                nodes = nodes_searched,
                nps = nodes_searched * 1000 / elapsed_millis,
                time = elapsed_millis,
                pv = rm.pv.iter().map(|m| m.to_usi_string()).collect::<Vec<_>>().join(" ")
            );
            Some(line)
        };
        let mut lines = self
            .root_moves
            .iter()
            .take(multi_pv)
            .enumerate()
            .flat_map(|(i, rm)| info_with_multi_pv_index(i, rm))
            .collect::<Vec<_>>();
        if reverse {
            lines.reverse();
        }
        lines.join("\n")
    }
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        ThreadPool {
            thread_pool_base: Arc::new(Mutex::new(ThreadPoolBase { threads: vec![] })),
            nodess: vec![],
            book: None,
            timeman: Arc::new(Mutex::new(TimeManagement::new())),
            best_previous_score: Arc::new(Mutex::new(Value::INFINITE)),
            iter_values: Arc::new(Mutex::new([Value::ZERO; 4])),
            best_move_changess: vec![],
            stop_on_ponderhit: Arc::new(AtomicBool::new(false)),
            ponder: Arc::new(AtomicBool::new(false)),
            stop: Arc::new(AtomicBool::new(false)),
            increase_depth: Arc::new(AtomicBool::new(true)),
            hide_all_output: Arc::new(AtomicBool::new(false)),
            limits: LimitsType::new(),
            last_best_root_move: Arc::new(Mutex::new(None)),
            handle: None,
        }
    }
    pub fn clear(&mut self) {
        for th in self.thread_pool_base.lock().unwrap().threads.iter() {
            th.lock().unwrap().clear();
        }
        *self.last_best_root_move.lock().unwrap() = None;

        let thread_pool_base = self.thread_pool_base.lock().unwrap();
        let mut main_thread = thread_pool_base.threads[0].lock().unwrap();
        main_thread.calls_count = 0;
        *main_thread.best_previous_score.lock().unwrap() = Value::INFINITE;
        main_thread.previous_time_reduction = 1.0;
    }
    pub fn set(
        &mut self,
        requested: usize,
        tt: &mut TranspositionTable,
        #[cfg(feature = "kppt")] ehash: &mut EvalHash,
        breadcrumbs: &mut Breadcrumbs,
        reductions: &mut Reductions,
    ) {
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
            self.thread_pool_base.lock().unwrap().threads.clear();
            self.nodess.clear();
        }
        self.thread_pool_base.lock().unwrap().threads.clear();
        self.nodess = (0..requested).map(|_| Arc::new(AtomicI64::new(0))).collect();
        self.best_move_changess = (0..requested).map(|_| Arc::new(AtomicU64::new(0))).collect();
        *reductions = Reductions::new(requested);
        self.thread_pool_base.lock().unwrap().threads = (0..requested)
            .map(|i| {
                Arc::new(Mutex::new(Thread {
                    idx: i,
                    pv_idx: 0,
                    tt_hit_average: 0,
                    sel_depth: 0,
                    null_move_pruning_min_ply: 0,
                    null_move_pruning_color: Color::BLACK,
                    position: Position::new(),
                    root_moves: RootMoves::new(),
                    root_depth: Depth::ZERO,
                    completed_depth: Depth::ZERO,
                    counter_moves: CounterMoveHistory::new(),
                    main_history: ButterflyHistory::new(),
                    low_ply_history: LowPlyHistory::new(),
                    capture_history: CapturePieceToHistory::new(),
                    continuation_history: [
                        [ContinuationHistory::new(), ContinuationHistory::new()],
                        [ContinuationHistory::new(), ContinuationHistory::new()],
                    ],
                    limits: self.limits.clone(),
                    tt,
                    timeman: self.timeman.clone(),
                    #[cfg(feature = "kppt")]
                    ehash,
                    breadcrumbs,
                    reductions,
                    usi_options: UsiOptions::new(),
                    best_move_changes: self.best_move_changess[i].clone(),
                    best_move_changess: self.best_move_changess.clone(),
                    nodes: self.nodess[i].clone(),
                    best_previous_score: self.best_previous_score.clone(),
                    iter_values: self.iter_values.clone(),
                    increase_depth: self.increase_depth.clone(),
                    previous_time_reduction: 1.0,
                    calls_count: 0,
                    stop_on_ponderhit: self.stop_on_ponderhit.clone(),
                    ponder: self.ponder.clone(),
                    stop: self.stop.clone(),
                    hide_all_output: self.hide_all_output.clone(),
                    nodess: vec![],
                }))
            })
            .collect();
        // Main thread has other thread's nodes.
        self.thread_pool_base.lock().unwrap().threads[0].lock().unwrap().nodess = self.nodess.clone();
    }
    pub fn start_thinking(
        &mut self,
        pos: &Position,
        tt: &mut TranspositionTable,
        limits: LimitsType,
        usi_options: &UsiOptions,
        ponder_mode: bool,
        hide_all_output: bool,
    ) {
        let mut limits = limits;
        if limits.perft.is_some() {
            Perft::new(pos).go(limits.perft.unwrap());
            return;
        }
        self.wait_for_search_finished();
        self.stop.store(false, Ordering::Relaxed);
        self.stop_on_ponderhit.store(false, Ordering::Relaxed);
        self.ponder.store(ponder_mode, Ordering::Relaxed);
        self.hide_all_output.store(hide_all_output, Ordering::Relaxed);
        self.timeman
            .lock()
            .unwrap()
            .init(usi_options, &mut limits, pos.side_to_move(), pos.ply());
        tt.new_search();
        self.limits = limits.clone();
        let root_moves = {
            let mut mlist = MoveList::new();
            mlist.generate::<LegalType>(pos, 0);
            let mut root_moves = RootMoves::new();
            let book_move = if usi_options.get_bool(UsiOptions::BOOK_ENABLE) {
                match &self.book {
                    Some(book) => book.probe(pos, &mut rand::thread_rng()),
                    None => None,
                }
            } else {
                None
            };
            match book_move {
                Some(book_move) => {
                    root_moves.push(RootMove::new(book_move));
                }
                None => {
                    for m in mlist.slice(0) {
                        root_moves.push(RootMove::new(m.mv));
                    }
                }
            }
            root_moves
        };
        let dummy_nodes = Arc::new(AtomicI64::new(0)); // This isn't used.
        let pos = Position::new_from_position(pos, dummy_nodes);
        let nodess_cloned = self.nodess.clone();
        let timeman_cloned = self.timeman.clone();
        let previous_score_cloned = self.best_previous_score.clone();
        let thread_pool_base_cloned = self.thread_pool_base.clone();
        let stop_cloned = self.stop.clone();
        let ponder_cloned = self.ponder.clone();
        let hide_all_output_cloned = self.hide_all_output.clone();
        let usi_options_cloned = usi_options.clone();
        let last_best_root_move_cloned = self.last_best_root_move.clone();
        self.handle = Some(
            std::thread::Builder::new()
                .stack_size(crate::stack_size::STACK_SIZE)
                .spawn(move || {
                    if root_moves.is_empty() || pos.is_entering_king_win() {
                        while !stop_cloned.load(Ordering::Relaxed)
                            && (ponder_cloned.load(Ordering::Relaxed) || limits.infinite.is_some())
                        {
                            std::thread::sleep(std::time::Duration::from_millis(1));
                        }
                        let m = if root_moves.is_empty() {
                            *last_best_root_move_cloned.lock().unwrap() = Some(RootMove::new(Move::RESIGN));
                            "resign"
                        } else {
                            *last_best_root_move_cloned.lock().unwrap() = Some(RootMove::new(Move::WIN));
                            "win"
                        };
                        if !hide_all_output_cloned.load(Ordering::Relaxed) {
                            println!("bestmove {}", m);
                        }
                        return;
                    }
                    let mut v = vec![];
                    for (i, thread) in thread_pool_base_cloned
                        .lock()
                        .unwrap()
                        .threads
                        .iter_mut()
                        .enumerate()
                        // i == 0 => not using a worker thread.
                        .rev()
                    {
                        let nodes_cloned = nodess_cloned[i].clone();
                        let pos = Position::new_from_position(&pos, nodes_cloned.clone());
                        nodes_cloned.store(0, Ordering::Relaxed);
                        let root_moves_cloned = root_moves.clone();
                        let thread_cloned = thread.clone();
                        let limits_cloned = limits.clone();
                        let usi_options_cloned = usi_options_cloned.clone();
                        let timeman_cloned = timeman_cloned.clone();
                        let worker = move || {
                            let mut th = thread_cloned.lock().unwrap();
                            th.best_move_changes.store(0, Ordering::Relaxed);
                            th.limits = limits_cloned;
                            th.nodes = nodes_cloned;
                            th.root_depth = Depth::ZERO;
                            th.root_moves = root_moves_cloned;
                            th.position = pos;
                            th.usi_options = usi_options_cloned;
                            th.timeman = timeman_cloned;
                            th.iterative_deepening_loop();
                        };
                        if i == 0 {
                            worker(); // The main thread doesn't use std::thread::spawn().
                        } else {
                            v.push(
                                std::thread::Builder::new()
                                    .stack_size(crate::stack_size::STACK_SIZE)
                                    .spawn(worker)
                                    .unwrap(),
                            );
                        }
                    }
                    while !stop_cloned.load(Ordering::Relaxed)
                        && (ponder_cloned.load(Ordering::Relaxed) || limits.infinite.is_some())
                    {
                        // nop
                    }
                    // main thread finished.
                    // stop the other threads.
                    stop_cloned.store(true, Ordering::Relaxed);
                    for handle in v {
                        handle.join().unwrap();
                    }

                    let multi_pv = std::cmp::min(usi_options_cloned.get_i64(UsiOptions::MULTI_PV) as usize, root_moves.len());
                    let best_thread = if multi_pv == 1 && limits.depth.is_none() && !root_moves.is_empty() {
                        let mut votes = std::collections::BTreeMap::new();
                        let min_score: Value = thread_pool_base_cloned
                            .lock()
                            .unwrap()
                            .threads
                            .iter()
                            .map(|x| x.lock().unwrap().root_moves[0].score)
                            .min()
                            .unwrap();

                        for th in thread_pool_base_cloned.lock().unwrap().threads.iter() {
                            let th = th.lock().unwrap();
                            *votes.entry(th.root_moves[0].pv[0].0.get()).or_insert(0) +=
                                i64::from((th.root_moves[0].score.0 - min_score.0 + 14) * th.completed_depth.0);
                        }

                        thread_pool_base_cloned
                            .lock()
                            .unwrap()
                            .threads
                            .iter()
                            // get first "max" score.
                            .min_by(|x, y| {
                                let x_score = x.lock().unwrap().root_moves[0].score;
                                let y_score = y.lock().unwrap().root_moves[0].score;
                                if x_score >= Value::MATE_IN_MAX_PLY || y_score >= Value::MATE_IN_MAX_PLY {
                                    y_score.cmp(&x_score)
                                } else {
                                    let x_vote_score = *votes.get(&x.lock().unwrap().root_moves[0].pv[0].0.get()).unwrap();
                                    let y_vote_score = *votes.get(&y.lock().unwrap().root_moves[0].pv[0].0.get()).unwrap();
                                    y_vote_score.cmp(&x_vote_score)
                                }
                            })
                            .unwrap()
                            .clone()
                    } else {
                        thread_pool_base_cloned.lock().unwrap().threads[0].clone()
                    };

                    *previous_score_cloned.lock().unwrap() = best_thread.lock().unwrap().root_moves[0].score;

                    let nodes_searched = thread_pool_base_cloned.lock().unwrap().threads[0]
                        .lock()
                        .unwrap()
                        .nodes_searched();
                    if let Ok(best_thread) = best_thread.lock() {
                        if !hide_all_output_cloned.load(Ordering::Relaxed) {
                            // Always send again PV info.
                            println!(
                                "{}",
                                best_thread.pv_info_to_usi_string(
                                    nodes_searched,
                                    multi_pv,
                                    best_thread.completed_depth,
                                    -Value::INFINITE,
                                    Value::INFINITE,
                                    true,
                                )
                            );
                            let mut s = format!("bestmove {}", best_thread.root_moves[0].pv[0].to_usi_string(),);
                            if usi_options_cloned.get_bool(UsiOptions::USI_PONDER) && best_thread.root_moves[0].pv.len() >= 2 {
                                s += &format!(" ponder {}", best_thread.root_moves[0].pv[1].to_usi_string());
                            }
                            println!("{}", s);
                        }
                    }
                    *last_best_root_move_cloned.lock().unwrap() = Some(best_thread.lock().unwrap().root_moves[0].clone());
                })
                .unwrap(),
        );
    }
    pub fn wait_for_search_finished(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
    #[allow(dead_code)]
    fn nodes_searched(&self) -> i64 {
        self.nodess.iter().fold(0, |sum, nodes| sum + nodes.load(Ordering::Relaxed))
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.wait_for_search_finished();
    }
}

#[test]
fn test_start_thinking() {
    std::thread::Builder::new()
        .stack_size(crate::stack_size::STACK_SIZE)
        .spawn(|| {
            let mut thread_pool = ThreadPool::new();
            let mut tt = TranspositionTable::new();
            #[cfg(feature = "kppt")]
            let usi_options = UsiOptions::new();
            #[cfg(feature = "kppt")]
            let mut ehash = EvalHash::new();
            tt.resize(16, &mut thread_pool);
            #[cfg(feature = "kppt")]
            ehash.resize(16, &mut thread_pool);
            #[cfg(feature = "kppt")]
            match load_evaluate_files(&usi_options.get_string(UsiOptions::EVAL_DIR)) {
                Ok(_) => {
                    let limits = {
                        let mut limits = LimitsType::new();
                        limits.depth = Some(1);
                        limits.start_time = Some(std::time::Instant::now());
                        limits
                    };
                    let mut breadcrumbs = Breadcrumbs::new();
                    let mut reductions = Reductions::new(1);
                    thread_pool.set(
                        3,
                        &mut tt,
                        #[cfg(feature = "kppt")]
                        &mut ehash,
                        &mut breadcrumbs,
                        &mut reductions,
                    );
                    let ponder_mode = false;
                    let hide_all_output = false;
                    thread_pool.start_thinking(&Position::new(), &mut tt, limits, &usi_options, ponder_mode, hide_all_output);
                    thread_pool.wait_for_search_finished();
                }
                Err(_) => {
                    // No evaluation funciton binaries.
                    // We want to do "cargo test" without evaluation function binaries.
                    // Then we do nothing and pass this test.
                    // todo: Is there a more better way?
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}
