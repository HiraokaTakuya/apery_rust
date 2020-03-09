use crate::evaluate::*;
use crate::movegen::*;
use crate::movepick::*;
use crate::movetypes::*;
use crate::position::*;
use crate::tt::*;
use crate::types::*;

pub const COUNTER_MOVE_PRUNE_THRESHOLD: i16 = 0;
pub const CURRENT_STACK_INDEX: usize = 7;
pub type Pv = True;
pub type NonPv = False;

pub const TT_HIT_AVERAGE_WINDOW: u64 = 4096;
pub const TT_HIT_AVERAGE_RESOLUTION: u64 = 1024;

#[derive(Clone)]
pub struct LimitsType {
    pub time: [std::time::Duration; 2],
    pub inc: [std::time::Duration; 2],
    pub depth: Option<u32>,
    pub movetime: Option<std::time::Duration>,
    pub mate: Option<u32>,
    pub perft: Option<u32>,
    pub infinite: Option<()>, // Is bool more appropriate?
    pub nodes: Option<u64>,
    pub start_time: Option<std::time::Instant>,
}

impl LimitsType {
    pub fn new() -> LimitsType {
        let duration = std::time::Duration::from_millis(0);
        LimitsType {
            time: [duration; 2],
            inc: [duration; 2],
            depth: None,
            movetime: None,
            mate: None,
            perft: None,
            infinite: None,
            nodes: None,
            start_time: None,
        }
    }
    pub fn use_time_management(&self) -> bool {
        self.mate.is_none()
            && self.movetime.is_none()
            && self.depth.is_none()
            && self.nodes.is_none()
            && self.perft.is_none()
            && self.infinite.is_none()
    }
}

#[derive(Clone, Eq)]
pub struct RootMove {
    pub score: Value,
    pub previous_score: Value,
    pub sel_depth: i32,
    pub best_move_count: usize,
    pub pv: Vec<Move>,
}

impl std::cmp::Ord for RootMove {
    fn cmp(&self, other: &RootMove) -> std::cmp::Ordering {
        match self.score.cmp(&other.score) {
            std::cmp::Ordering::Equal => self.previous_score.cmp(&other.previous_score),
            ord => ord,
        }
    }
}
impl std::cmp::PartialOrd for RootMove {
    fn partial_cmp(&self, other: &RootMove) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl std::cmp::PartialEq for RootMove {
    fn eq(&self, other: &RootMove) -> bool {
        self.score == other.score && self.previous_score == other.previous_score
    }
}

impl RootMove {
    pub fn new(m: Move) -> RootMove {
        RootMove {
            score: -Value::INFINITE,
            previous_score: -Value::INFINITE,
            sel_depth: 0,
            best_move_count: 0,
            pv: vec![m],
        }
    }
    pub fn extract_pv_from_tt(&mut self, pos: &mut Position, tt: *mut TranspositionTable) {
        let mut m = self.pv[0];
        debug_assert!(pos.pseudo_legal::<SearchingType>(m));
        let mut ply = 0;
        self.pv.clear();
        while {
            self.pv.push(m);
            debug_assert!(pos.pseudo_legal::<SearchingType>(m));
            let gives_check = pos.gives_check(m);
            pos.do_move(m, gives_check);
            ply += 1;
            let key = pos.key();
            let (tte, tt_hit) = unsafe { (*tt).probe(key) };
            tt_hit
                && {
                    if let Some(tt_move) = tte.mv(pos) {
                        m = tt_move;
                        debug_assert!(pos.pseudo_legal::<SearchingType>(m));
                        true
                    } else {
                        false
                    }
                }
                && pos.legal(m)
                && ply < MAX_PLY
                && pos.is_repetition() == Repetition::Not
        } {}
        for m in self.pv[..ply as usize].iter().rev() {
            pos.undo_move(*m);
        }
    }
}

pub type RootMoves = Vec<RootMove>;

#[derive(Clone, Copy)]
pub struct Stack {
    pub continuation_history: *mut PieceToHistory,
    pub ply: i32,
    pub current_move: Option<Move>,
    pub excluded_move: Option<Move>,
    pub killers: [Option<Move>; 2],
    pub static_eval: Value,
    pub static_eval_raw: EvalSum,
    pub stat_score: i32,
    pub move_count: i32,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            continuation_history: std::ptr::null_mut(),
            ply: 0,
            current_move: None,
            excluded_move: None,
            killers: [None, None],
            static_eval: Value::ZERO,
            static_eval_raw: EvalSum::new(),
            stat_score: 0,
            move_count: 0,
        }
    }
}

pub fn get_stack(stack: &[Stack], i: i64) -> &Stack {
    debug_assert!(((CURRENT_STACK_INDEX as i64 + i) as usize) < stack.len());
    unsafe { stack.get_unchecked((CURRENT_STACK_INDEX as i64 + i) as usize) }
}

pub fn get_stack_mut(stack: &mut [Stack], i: i64) -> &mut Stack {
    debug_assert!(((CURRENT_STACK_INDEX as i64 + i) as usize) < stack.len());
    unsafe { stack.get_unchecked_mut((CURRENT_STACK_INDEX as i64 + i) as usize) }
}

pub fn value_from_tt(v: Value, ply: i32) -> Value {
    match v {
        Value::NONE => Value::NONE,
        v if v >= Value::MATE_IN_MAX_PLY => v - Value(ply),
        v if v <= Value::MATED_IN_MAX_PLY => v + Value(ply),
        v => v,
    }
}

pub fn value_to_tt(v: Value, ply: i32) -> Value {
    debug_assert!(v != Value::NONE);
    match v {
        v if v >= Value::MATE_IN_MAX_PLY => v + Value(ply),
        v if v <= Value::MATED_IN_MAX_PLY => v - Value(ply),
        v => v,
    }
}

pub fn value_mate_in(ply: i32) -> Value {
    Value::MATE - Value(ply)
}

pub fn value_mated_in(ply: i32) -> Value {
    -Value::MATE + Value(ply)
}

pub const TEMPO: Value = Value(28);

pub fn stat_bonus(depth: Depth) -> i32 {
    let d = depth.0 / Depth::ONE_PLY.0;
    if d > 17 {
        -8
    } else {
        22 * d * d + 151 * d - 140
    }
}

pub fn value_draw(nodes: i64) -> Value {
    Value::DRAW + Value(2 * (nodes as i32 & 1) - 1)
}

pub fn update_continuation_histories(stack: &mut [Stack], pc: Piece, to: Square, bonus: i32) {
    for i in [2i64, 3, 5, 7].iter() {
        let m = get_stack(stack, -i).current_move;
        if m.is_normal_move() {
            unsafe {
                (*get_stack_mut(stack, -i).continuation_history).update(to, pc, bonus);
            }
        }
    }
}

pub const RAZOR_MARGIN: Value = Value(661);

pub fn futility_margin(depth: Depth) -> Value {
    Value(75 * depth.0 / Depth::ONE_PLY.0)
}

pub fn futility_move_count(improving: bool, depth_per_one_ply: i32) -> i32 {
    (5 + depth_per_one_ply * depth_per_one_ply) * (1 + i32::from(improving)) / 2
}

pub struct Reductions {
    values: [i32; ExtMove::MAX_LEGAL_MOVES],
}

impl Reductions {
    pub fn new(thread_num: usize) -> Reductions {
        let mut r = Reductions {
            values: [0; ExtMove::MAX_LEGAL_MOVES],
        };
        for (i, v) in r.values.iter_mut().enumerate().skip(1) {
            *v = ((23.4 + f64::from(thread_num as i32).ln()) * f64::from(i as i32).ln()) as i32;
        }
        r
    }
    pub fn get(&self, improving: bool, depth: Depth, move_count: i32) -> Depth {
        let r = unsafe {
            self.values
                .get_unchecked((depth.0 / Depth::ONE_PLY.0) as usize)
                * self.values.get_unchecked(move_count as usize)
        };
        Depth(((r + 520) / 1024 + i32::from(!improving && r > 999)) * Depth::ONE_PLY.0)
    }
}

pub const SKIP_SIZE: [i32; 20] = [1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4];
pub const SKIP_PHASE: [i32; 20] = [0, 1, 0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6, 7];

pub struct Perft {
    position: Position,
}

impl Perft {
    pub fn new(pos: &Position) -> Perft {
        Perft {
            position: Position::new_from_position(
                pos,
                std::sync::Arc::new(std::sync::atomic::AtomicI64::new(0)),
            ),
        }
    }
    pub fn go(&mut self, depth: u32) {
        let timeins1 = std::time::Instant::now();
        let searched_nodes = self.perft::<True>(depth);
        let timeins2 = std::time::Instant::now();
        let timedur = timeins2 - timeins1;
        let timedur_nanos = std::cmp::max(timedur.as_nanos(), 1);
        let moved_nodes = self.position.nodes_searched();
        println!();
        println!("Time duration: {:?}", timedur);
        println!(
            "Searched: {} nodes : {} nps",
            searched_nodes,
            (searched_nodes as u128) * 1_000_000_000 / timedur_nanos
        );
        println!(
            "(Moved: {} nodes : {} nps)",
            moved_nodes,
            (moved_nodes as u128) * 1_000_000_000 / timedur_nanos
        );
    }
    // perft() is our utility to verify move generation. All the leaf nodes up
    // to the given depth are generated and counted, and the sum is returned.
    pub fn perft<Root: Bool>(&mut self, depth: u32) -> u64 {
        let leaf: bool = depth == 2;
        let mut nodes: u64 = 0;
        let mut mlist = MoveList::new();
        mlist.generate::<LegalAllType>(&self.position, 0);
        for i in 0..mlist.size {
            let cnt: u64;
            if Root::BOOL && depth <= 1 {
                cnt = 1;
                nodes += 1;
            } else {
                let m = mlist.ext_moves[i].mv;
                self.position.do_move(m, self.position.gives_check(m));
                if leaf {
                    let mut leaf_mlist = MoveList::new();
                    leaf_mlist.generate::<LegalAllType>(&self.position, 0);
                    cnt = leaf_mlist.size as u64;
                } else {
                    cnt = self.perft::<False>(depth - 1);
                }
                nodes += cnt;
                self.position.undo_move(m);
            }
            if Root::BOOL {
                println!("{} : {}", mlist.ext_moves[i].mv.to_usi_string(), cnt);
            }
        }
        nodes
    }
}
