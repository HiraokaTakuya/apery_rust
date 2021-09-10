use crate::movegen::*;
use crate::movetypes::*;
use crate::piecevalue::*;
use crate::position::*;
use crate::types::*;

fn partial_insertion_sort(move_list: &mut [ExtMove], limit: i32) {
    let mut sorted_end = 0;
    for p in 1..move_list.len() {
        unsafe {
            if move_list.get_unchecked(p).score >= limit {
                let tmp = move_list.get_unchecked(p).clone();
                sorted_end += 1;
                *move_list.get_unchecked_mut(p) = move_list.get_unchecked(sorted_end).clone();
                let mut q = sorted_end;
                while q != 0 && move_list.get_unchecked(q - 1).score < tmp.score {
                    *move_list.get_unchecked_mut(q) = move_list.get_unchecked(q - 1).clone();
                    q -= 1;
                }
                *move_list.get_unchecked_mut(q) = tmp;
            }
        }
    }
}

pub struct ButterflyHistory {
    v: [[i16; 0xffff]; Color::NUM], // using lower 16bit of Move as array index of v.
}

impl ButterflyHistory {
    pub fn new() -> ButterflyHistory {
        ButterflyHistory {
            v: [[0; 0xffff]; Color::NUM],
        }
    }
    pub fn get(&self, c: Color, m: Move) -> i32 {
        i32::from(self.v[c.0 as usize][m.0.get() as u16 as usize])
    }
    pub fn update(&mut self, c: Color, m: Move, bonus: i32) {
        let entry = &mut self.v[c.0 as usize][m.0.get() as u16 as usize];
        let mut val = *entry;
        val += (bonus - i32::from(val) * bonus.abs() / 13365) as i16;
        *entry = val;
    }
    pub fn fill(&mut self, val: i16) {
        for x in self.v.iter_mut() {
            for y in x.iter_mut() {
                *y = val;
            }
        }
    }
}

pub struct LowPlyHistory {
    v: [[i16; 0xffff]; Self::MAX_LPH], // using lower 16bit of Move as array index of v.
}

impl LowPlyHistory {
    pub const MAX_LPH: usize = 4;
    pub fn new() -> Self {
        Self {
            v: [[0; 0xffff]; Self::MAX_LPH],
        }
    }
    pub fn get(&self, ply: i32, m: Move) -> i32 {
        i32::from(self.v[ply as usize][m.0.get() as u16 as usize])
    }
    pub fn update(&mut self, ply: i32, m: Move, bonus: i32) {
        let entry = &mut self.v[ply as usize][m.0.get() as u16 as usize];
        let mut val = *entry;
        val += (bonus - i32::from(val) * bonus.abs() / 10692) as i16;
        *entry = val;
    }
    pub fn fill(&mut self, val: i16) {
        for x in self.v.iter_mut() {
            for y in x.iter_mut() {
                *y = val;
            }
        }
    }
    pub fn keep_data_from_previous_search(&mut self) {
        for ply in 2..Self::MAX_LPH {
            self.v[ply - 2] = self.v[ply];
        }
        for ply in Self::MAX_LPH - 2..Self::MAX_LPH {
            self.v[ply].iter_mut().for_each(|item| *item = 0);
        }
    }
}

pub struct CounterMoveHistory {
    v: [[Option<Move>; Piece::NUM]; Square::NUM],
}

impl CounterMoveHistory {
    pub fn new() -> CounterMoveHistory {
        CounterMoveHistory {
            v: [[None; Piece::NUM]; Square::NUM],
        }
    }
    pub fn get(&self, to: Square, pc: Piece) -> Option<Move> {
        self.v[to.0 as usize][pc.0 as usize]
    }
    pub fn set(&mut self, to: Square, pc: Piece, m: Move) {
        self.v[to.0 as usize][pc.0 as usize] = Some(m);
    }
    pub fn fill(&mut self, m: Option<Move>) {
        for x in self.v.iter_mut() {
            for y in x.iter_mut() {
                *y = m;
            }
        }
    }
}

pub struct CapturePieceToHistory {
    v: [[[i16; PieceType::NUM]; Square::NUM]; Piece::NUM],
}

impl CapturePieceToHistory {
    pub fn new() -> CapturePieceToHistory {
        CapturePieceToHistory {
            v: [[[0; PieceType::NUM]; Square::NUM]; Piece::NUM],
        }
    }
    pub fn get(&self, pc: Piece, to: Square, captured: PieceType) -> i32 {
        i32::from(self.v[pc.0 as usize][to.0 as usize][captured.0 as usize])
    }
    pub fn update(&mut self, pc: Piece, to: Square, captured: PieceType, bonus: i32) {
        let entry = &mut self.v[pc.0 as usize][to.0 as usize][captured.0 as usize];
        let mut val = *entry;
        val += (bonus - i32::from(val) * bonus.abs() / 10692) as i16;
        *entry = val;
    }
    pub fn fill(&mut self, val: i16) {
        for x in self.v.iter_mut() {
            for y in x.iter_mut() {
                for z in y.iter_mut() {
                    *z = val;
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct PieceToHistory {
    v: [[i16; Piece::NUM]; Square::NUM],
}

impl PieceToHistory {
    #[allow(dead_code)]
    pub fn new() -> PieceToHistory {
        PieceToHistory {
            v: [[0; Piece::NUM]; Square::NUM],
        }
    }
    pub fn get(&self, to: Square, pc: Piece) -> i32 {
        i32::from(self.v[to.0 as usize][pc.0 as usize])
    }
    pub fn update(&mut self, to: Square, pc: Piece, bonus: i32) {
        let entry = &mut self.v[to.0 as usize][pc.0 as usize];
        let mut val = *entry;
        val += (bonus - i32::from(val) * bonus.abs() / 29952) as i16;
        *entry = val;
    }
    pub fn fill(&mut self, val: i16) {
        for x in self.v.iter_mut() {
            for y in x.iter_mut() {
                *y = val;
            }
        }
    }
}

pub struct ContinuationHistory {
    pub v: [[PieceToHistory; Square::NUM]; Piece::NUM],
}

impl ContinuationHistory {
    pub fn new() -> Self {
        Self {
            v: [[PieceToHistory::new(); Square::NUM]; Piece::NUM],
        }
    }
    #[allow(dead_code)]
    pub fn get(&self, pc: Piece, to: Square) -> &PieceToHistory {
        &self.v[pc.0 as usize][to.0 as usize]
    }
    pub fn get_mut(&mut self, pc: Piece, to: Square) -> &mut PieceToHistory {
        &mut self.v[pc.0 as usize][to.0 as usize]
    }
    pub fn fill(&mut self, val: i16) {
        for x in self.v.iter_mut() {
            for y in x.iter_mut() {
                y.fill(val);
            }
        }
    }
    pub fn sentinel(&mut self) -> &mut PieceToHistory {
        self.get_mut(Piece::EMPTY, Square(0))
    }
}

custom_derive! {
    #[derive(Debug, NextVariant)]
    enum StagesForMainSearch {
        MainTt, CaptureInit, GoodCapture, Refutation, QuietInit, Quiet, BadCapture,
        EvasionTt, EvasionInit, Evasion,
    }
}
custom_derive! {
    #[derive(Debug, NextVariant)]
    enum StagesForQSearch {
        QSearchTt, QCaptureInit, QCapture, /*QCheckInit, QCheck,*/
        QRecaptureTt, QRecaptureInit, QRecapture,
        EvasionTt, EvasionInit, Evasion,
    }
}
custom_derive! {
    #[derive(Debug, NextVariant)]
    enum StagesForProbCut {
        Tt, Init, ProbCut,
    }
}

fn pick_best(list: &mut [ExtMove]) -> Move {
    // get first max.
    let (max_index, _max_item) = list.iter().enumerate().min_by(|x, y| y.1.cmp(x.1)).unwrap();
    list.swap(0, max_index);
    list[0].mv
}

fn select_next_refutation(
    list: &[Option<Move>],
    current_index: &mut usize,
    pos: &Position,
    tt_move: Option<Move>,
) -> Option<Move> {
    for &mv in list {
        *current_index += 1;
        if let Some(m) = mv {
            if m != tt_move.non_zero_unwrap_unchecked() && !m.is_capture(pos) && pos.pseudo_legal::<SearchingType>(m) {
                return Some(m);
            }
        }
    }
    None
}

fn select_best_good_capture(
    ext_moves: &mut [ExtMove],
    ext_moves_size: usize,
    current_index: &mut usize,
    end_bad_captures: &mut usize,
    pos: &Position,
    tt_move: Option<Move>,
) -> Option<Move> {
    while *current_index < ext_moves_size {
        let m = pick_best(&mut ext_moves[*current_index..ext_moves_size]);
        let score = ext_moves[*current_index].score;
        *current_index += 1;
        if m != tt_move.non_zero_unwrap_unchecked() {
            if pos.see_ge(m, Value(-69 * score / 1024)) {
                return Some(m);
            } else {
                ext_moves[*end_bad_captures].mv = m;
                *end_bad_captures += 1;
            }
        }
    }
    None
}

fn select_next_quiet(
    list: &[ExtMove],
    current_index: &mut usize,
    refutations: &[Option<Move>],
    tt_move: Option<Move>,
) -> Option<Move> {
    for ext_move in list {
        *current_index += 1;
        let m = ext_move.mv;
        if m != tt_move.non_zero_unwrap_unchecked()
            && m != refutations[0].non_zero_unwrap_unchecked()
            && m != refutations[1].non_zero_unwrap_unchecked()
            && m != refutations[2].non_zero_unwrap_unchecked()
        {
            return Some(m);
        }
    }
    None
}

fn select_next_bad_capture(list: &[ExtMove], current_index: &mut usize, tt_move: Option<Move>) -> Option<Move> {
    for ext_move in list {
        *current_index += 1;
        let m = ext_move.mv;
        if m != tt_move.non_zero_unwrap_unchecked() {
            return Some(m);
        }
    }
    None
}

fn select_best_evasion(list: &mut [ExtMove], current_index: &mut usize, tt_move: Option<Move>) -> Option<Move> {
    for i in 0..list.len() {
        let m = pick_best(&mut list[i..]);
        *current_index += 1;
        if m != tt_move.non_zero_unwrap_unchecked() {
            return Some(m);
        }
    }
    None
}

fn select_best_qcapture(list: &mut [ExtMove], current_index: &mut usize, tt_move: Option<Move>) -> Option<Move> {
    for i in 0..list.len() {
        let m = pick_best(&mut list[i..]);
        *current_index += 1;
        if m != tt_move.non_zero_unwrap_unchecked() {
            return Some(m);
        }
    }
    None
}

fn select_best_qrecapture(list: &mut [ExtMove], current_index: &mut usize, tt_move: Option<Move>) -> Option<Move> {
    for i in 0..list.len() {
        let m = pick_best(&mut list[i..]);
        *current_index += 1;
        if m != tt_move.non_zero_unwrap_unchecked() {
            return Some(m);
        }
    }
    None
}

fn select_best_probcut(
    list: &mut [ExtMove],
    current_index: &mut usize,
    tt_move: Option<Move>,
    pos: &Position,
    threshold: Value,
) -> Option<Move> {
    for i in 0..list.len() {
        let m = pick_best(&mut list[i..]);
        *current_index += 1;
        if m != tt_move.non_zero_unwrap_unchecked() && pos.see_ge(m, threshold) {
            return Some(m);
        }
    }
    None
}

fn score_captures(move_list: &mut [ExtMove], pos: &Position, capture_history: *const CapturePieceToHistory) {
    for ext_move in move_list {
        let m = ext_move.mv;
        let to = m.to();
        let pc_to = pos.piece_on(to);
        let pt_to = PieceType::new(pc_to);
        ext_move.score =
            capture_piece_type_value(pt_to).0 + unsafe { (*capture_history).get(m.piece_moved_after_move(), to, pt_to) };
    }
}

fn score_recaptures(move_list: &mut [ExtMove], pos: &Position) {
    for ext_move in move_list {
        let m = ext_move.mv;
        ext_move.score = (capture_piece_value(pos.piece_on(m.to())) - lva_value(PieceType::new(m.piece_moved_before_move()))).0;
    }
}

fn score_quiets(
    move_list: &mut [ExtMove],
    pos: &Position,
    main_history: *const ButterflyHistory,
    low_ply_history: *const LowPlyHistory,
    continuation_history: &[*const PieceToHistory],
    ply: i32,
) {
    for ext_move in move_list {
        let m = ext_move.mv;
        let to = m.to();
        let piece_moved = m.piece_moved_after_move();
        let side_to_move = pos.side_to_move();
        ext_move.score = unsafe { (*main_history).get(side_to_move, m) }
            + 2 * unsafe { (*continuation_history[0]).get(to, piece_moved) }
            + unsafe { (*continuation_history[1]).get(to, piece_moved) }
            + unsafe { (*continuation_history[3]).get(to, piece_moved) }
            + unsafe { (*continuation_history[5]).get(to, piece_moved) }
            + if ply < LowPlyHistory::MAX_LPH as i32 {
                6 * unsafe { (*low_ply_history).get(ply, m) }
            } else {
                0
            }
    }
}

fn score_evasion(
    move_list: &mut [ExtMove],
    pos: &Position,
    main_history: *const ButterflyHistory,
    continuation_history: &[*const PieceToHistory],
) {
    for ext_move in move_list {
        let m = ext_move.mv;
        if m.is_capture(pos) {
            ext_move.score =
                (capture_piece_value(pos.piece_on(m.to())) - lva_value(PieceType::new(m.piece_moved_before_move()))).0;
        } else {
            let piece_moved = m.piece_moved_after_move();
            ext_move.score = unsafe { (*main_history).get(pos.side_to_move(), m) }
                + 2 * unsafe { (*continuation_history[0]).get(m.to(), piece_moved) }
                - (1 << 28);
        }
    }
}

pub struct MovePickerForMainSearch<'a> {
    main_history: *const ButterflyHistory,
    low_ply_history: *const LowPlyHistory,
    capture_history: *const CapturePieceToHistory,
    continuation_history: &'a [*const PieceToHistory],
    cur: usize,
    end_bad_captures: usize,
    stage: StagesForMainSearch,
    tt_move: Option<Move>,
    refutations: [Option<Move>; 3],
    refutations_size: usize,
    depth: Depth,
    move_list: MoveList,
    ply: i32,
}

impl<'a> MovePickerForMainSearch<'a> {
    pub fn new(
        pos: &Position,
        ttm: Option<Move>,
        depth: Depth,
        mh: &ButterflyHistory,
        lph: &LowPlyHistory,
        cph: &CapturePieceToHistory,
        ch: &'a [*const PieceToHistory],
        cm: Option<Move>,
        killers: &[Option<Move>],
        ply: i32,
    ) -> MovePickerForMainSearch<'a> {
        let mut stage = if pos.in_check() {
            StagesForMainSearch::EvasionTt
        } else {
            StagesForMainSearch::MainTt
        };
        match ttm {
            Some(ttm_inner) => {
                // This move has already been checked for Position::pseudo_legal.
                debug_assert!(pos.pseudo_legal::<SearchingType>(ttm_inner));
            }
            _ => {
                stage = stage.next_variant().unwrap();
            }
        }
        MovePickerForMainSearch {
            main_history: mh,
            low_ply_history: lph,
            capture_history: cph,
            continuation_history: ch,
            cur: 0,
            end_bad_captures: 0,
            stage,
            tt_move: ttm,
            refutations: [killers[0], killers[1], cm],
            refutations_size: 3,
            depth,
            move_list: MoveList::new(),
            ply,
        }
    }
    pub fn next_move(&mut self, pos: &Position, skip_quiets: bool) -> Option<Move> {
        loop {
            match self.stage {
                StagesForMainSearch::MainTt | StagesForMainSearch::EvasionTt => {
                    self.stage = self.stage.next_variant().unwrap();
                    return self.tt_move;
                }
                StagesForMainSearch::CaptureInit => {
                    self.move_list.generate::<CaptureOrPawnPromotionsType>(pos, 0);
                    score_captures(self.move_list.slice_mut(0), pos, self.capture_history);
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForMainSearch::GoodCapture => {
                    let size = self.move_list.size;
                    if let Some(m) = select_best_good_capture(
                        self.move_list.slice_mut(0),
                        size,
                        &mut self.cur,
                        &mut self.end_bad_captures,
                        pos,
                        self.tt_move,
                    ) {
                        return Some(m);
                    }
                    self.cur = 0;

                    // if countermove == killer, skip it.
                    if self.refutations[0] == self.refutations[2] || self.refutations[1] == self.refutations[2] {
                        self.refutations_size = 2;
                    }
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForMainSearch::Refutation => {
                    if let Some(m) = select_next_refutation(
                        &self.refutations[self.cur..self.refutations_size],
                        &mut self.cur,
                        pos,
                        self.tt_move,
                    ) {
                        return Some(m);
                    }
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForMainSearch::QuietInit => {
                    if !skip_quiets {
                        self.cur = self.end_bad_captures;
                        self.move_list.generate::<QuietsWithoutPawnPromotionsType>(pos, self.cur);
                        score_quiets(
                            self.move_list.slice_mut(self.cur),
                            pos,
                            self.main_history,
                            self.low_ply_history,
                            self.continuation_history,
                            self.ply,
                        );
                        partial_insertion_sort(self.move_list.slice_mut(self.cur), -3000 * self.depth.0);
                    }
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForMainSearch::Quiet => {
                    if !skip_quiets {
                        if let Some(m) =
                            select_next_quiet(self.move_list.slice(self.cur), &mut self.cur, &self.refutations, self.tt_move)
                        {
                            return Some(m);
                        }
                    }
                    self.cur = 0;
                    self.move_list.size = self.end_bad_captures;
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForMainSearch::BadCapture => {
                    return select_next_bad_capture(self.move_list.slice(self.cur), &mut self.cur, self.tt_move);
                }
                StagesForMainSearch::EvasionInit => {
                    self.cur = 0;
                    self.move_list.generate::<EvasionsType>(pos, 0);
                    score_evasion(self.move_list.slice_mut(0), pos, self.main_history, self.continuation_history);
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForMainSearch::Evasion => {
                    return select_best_evasion(self.move_list.slice_mut(self.cur), &mut self.cur, self.tt_move);
                }
            }
        }
    }
}

pub struct MovePickerForQSearch<'a> {
    main_history: *const ButterflyHistory,
    capture_history: *const CapturePieceToHistory,
    continuation_history: &'a [*const PieceToHistory],
    cur: usize,
    recapture_square: Square,
    stage: StagesForQSearch,
    tt_move: Option<Move>,
    move_list: MoveList,
}

impl<'a> MovePickerForQSearch<'a> {
    pub fn new(
        main_history: &ButterflyHistory,
        capture_history: &CapturePieceToHistory,
        continuation_history: &'a [*const PieceToHistory],
        pos: &Position,
        recapture_square: Square,
        ttm: Option<Move>,
        depth: Depth,
    ) -> MovePickerForQSearch<'a> {
        let in_check = pos.in_check();
        let mut stage = if in_check {
            StagesForQSearch::EvasionTt
        } else if depth > Depth::QS_RECAPTURES {
            StagesForQSearch::QSearchTt
        } else {
            StagesForQSearch::QRecaptureTt
        };
        match ttm {
            Some(ttm_inner) if (in_check || depth > Depth::QS_RECAPTURES || ttm_inner.to() == recapture_square) => {
                // This move has already been checked for Position::pseudo_legal.
                debug_assert!(pos.pseudo_legal::<SearchingType>(ttm_inner));
            }
            _ => {
                stage = stage.next_variant().unwrap();
            }
        }
        MovePickerForQSearch {
            main_history,
            capture_history,
            continuation_history,
            cur: 0,
            recapture_square,
            stage,
            tt_move: ttm,
            move_list: MoveList::new(),
        }
    }

    pub fn next_move(&mut self, pos: &Position) -> Option<Move> {
        loop {
            match self.stage {
                StagesForQSearch::QSearchTt | StagesForQSearch::EvasionTt | StagesForQSearch::QRecaptureTt => {
                    self.stage = self.stage.next_variant().unwrap();
                    return self.tt_move;
                }
                StagesForQSearch::QCaptureInit => {
                    self.move_list.generate::<CaptureOrPawnPromotionsType>(pos, 0);
                    score_captures(self.move_list.slice_mut(0), pos, self.capture_history);
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForQSearch::QRecaptureInit => {
                    self.move_list.generate_recaptures(pos, self.recapture_square);
                    score_recaptures(self.move_list.slice_mut(0), pos);
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForQSearch::QCapture => {
                    let m = select_best_qcapture(self.move_list.slice_mut(self.cur), &mut self.cur, self.tt_move);
                    return m;
                    //if m != Move::NONE {
                    //    return m;
                    //}
                    //if self.depth != Depth::QS_CHECKS {
                    //    return Move::NONE;
                    //}
                    //self.stage = self.stage.next_variant().unwrap();
                }
                StagesForQSearch::QRecapture => {
                    return select_best_qrecapture(self.move_list.slice_mut(self.cur), &mut self.cur, self.tt_move);
                }
                //StagesForQSearch::QCheckInit => {}
                //StagesForQSearch::QCheck => {}
                StagesForQSearch::EvasionInit => {
                    self.cur = 0;
                    self.move_list.generate::<EvasionsType>(pos, 0);
                    score_evasion(self.move_list.slice_mut(0), pos, self.main_history, self.continuation_history);
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForQSearch::Evasion => {
                    return select_best_evasion(self.move_list.slice_mut(self.cur), &mut self.cur, self.tt_move);
                }
            }
        }
    }
}

pub struct MovePickerForProbCut {
    capture_history: *const CapturePieceToHistory,
    cur: usize,
    stage: StagesForProbCut,
    tt_move: Option<Move>,
    threshold: Value,
    move_list: MoveList,
}

impl MovePickerForProbCut {
    pub fn new(pos: &Position, ttm: Option<Move>, thresh: Value, cph: &CapturePieceToHistory) -> MovePickerForProbCut {
        debug_assert!(!pos.in_check());
        let mut stage = StagesForProbCut::Tt;
        match ttm {
            Some(ttm_inner)
                if ttm_inner.is_capture(pos) && {
                    // This move has already been checked for Position::pseudo_legal.
                    debug_assert!(pos.pseudo_legal::<SearchingType>(ttm_inner));
                    pos.see_ge(ttm_inner, thresh)
                } => {}
            _ => {
                stage = stage.next_variant().unwrap();
            }
        }
        MovePickerForProbCut {
            capture_history: cph,
            cur: 0,
            stage,
            tt_move: ttm,
            threshold: thresh,
            move_list: MoveList::new(),
        }
    }

    pub fn next_move(&mut self, pos: &Position) -> Option<Move> {
        loop {
            match self.stage {
                StagesForProbCut::Tt => {
                    self.stage = self.stage.next_variant().unwrap();
                    return self.tt_move;
                }
                StagesForProbCut::Init => {
                    self.move_list.generate::<CaptureOrPawnPromotionsType>(pos, 0);
                    score_captures(self.move_list.slice_mut(0), pos, self.capture_history);
                    self.stage = self.stage.next_variant().unwrap();
                }
                StagesForProbCut::ProbCut => {
                    return select_best_probcut(
                        self.move_list.slice_mut(self.cur),
                        &mut self.cur,
                        self.tt_move,
                        pos,
                        self.threshold,
                    );
                }
            }
        }
    }
}

#[test]
fn test_partial_insertion_sort() {
    let mut ext_moves = vec![
        ExtMove {
            mv: Move::NULL,
            score: 0,
        },
        ExtMove {
            mv: Move::NULL,
            score: 9,
        },
        ExtMove {
            mv: Move::NULL,
            score: 11,
        },
        ExtMove {
            mv: Move::NULL,
            score: 15,
        },
        ExtMove {
            mv: Move::NULL,
            score: 3,
        },
        ExtMove {
            mv: Move::NULL,
            score: 13,
        },
        ExtMove {
            mv: Move::NULL,
            score: 6,
        },
    ];
    partial_insertion_sort(ext_moves.as_mut_slice(), 10);
    assert_eq!(ext_moves[0].score, 15);
    assert_eq!(ext_moves[1].score, 13);
    assert_eq!(ext_moves[2].score, 11);
}

#[test]
fn test_butterfly_history() {
    unsafe {
        let mut vec: Vec<ButterflyHistory> = vec![ButterflyHistory { v: [[0; 0xffff]; 2] }];
        let history = vec.as_mut_ptr();
        let bonus = 3;
        let m = Move::new_unpromote(Square::SQ77, Square::SQ76, Piece::B_PAWN);
        (*history).update(Color::BLACK, m, bonus);
        let v = (*history).get(Color::BLACK, m);
        assert_eq!(bonus, v);
    }
}

#[test]
fn test_move_list_select_best() {
    let sfen = "k8/9/3b1l3/4s4/5pg2/4GP3/5RN2/9/K4L3 b - 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let mut mlist = MoveList::new();
    mlist.generate::<CaptureOrPawnPromotionsType>(&pos, 0);
    let capture_history = CapturePieceToHistory::new();
    score_captures(mlist.slice_mut(0), &pos, &capture_history);
    let mut cur = 0;
    let mut end_bad_captures = 0;
    let size = mlist.size;
    let m = select_best_good_capture(mlist.slice_mut(0), size, &mut cur, &mut end_bad_captures, &pos, None);
    assert_eq!(m.unwrap().to_csa_string(&pos), "4645FU");

    let sfen = "k8/lpppppp2/rbgsnlp2/+RPPPPPP2/9/9/9/9/K8 b - 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let mut mlist = MoveList::new();
    mlist.generate::<CaptureOrPawnPromotionsType>(&pos, 0);
    let capture_history = CapturePieceToHistory::new();
    score_captures(mlist.slice_mut(0), &pos, &capture_history);
    let mut cur = 0;
    let mut end_bad_captures = 0;
    let size = mlist.size;
    let m = select_best_good_capture(mlist.slice_mut(0), size, &mut cur, &mut end_bad_captures, &pos, None);
    assert_eq!(m.unwrap().to_csa_string(&pos), "8483TO");
}

#[test]
fn test_move_picker_for_main_search_next_move() {
    let sfen = "k8/9/9/5b3/9/l8/p8/1B7/1K7 b - 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let tt_move = Some(Move::new_unpromote(Square::SQ88, Square::SQ66, Piece::B_BISHOP));
    let mh = ButterflyHistory::new();
    let lph = LowPlyHistory::new();
    let cph = CapturePieceToHistory::new();
    let ch = [
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
    ];
    let ch = ch.iter().map(|x| x as *const PieceToHistory).collect::<Vec<_>>();
    let killers = [
        Some(Move::new_unpromote(Square::SQ88, Square::SQ99, Piece::B_BISHOP)),
        Some(Move::new_unpromote(Square::SQ88, Square::SQ79, Piece::B_BISHOP)),
    ];
    let cm = Some(Move::new_unpromote(Square::SQ88, Square::SQ77, Piece::B_BISHOP));
    let skip_quiets = false;
    let mut mp = MovePickerForMainSearch::new(&pos, tt_move, Depth(5), &mh, &lph, &cph, &ch, cm, &killers, 0);
    let moves_size;
    {
        let mut mlist = MoveList::new();
        mlist.generate::<NonEvasionsType>(&pos, 0);
        moves_size = mlist.size;
    }
    assert_eq!(moves_size, 11); // legal: 10, illegal: 1
    let mut move_vec = vec![];
    for _ in 0..moves_size {
        let m = mp.next_move(&pos, skip_quiets);
        move_vec.push(m);
    }
    assert_eq!(move_vec[0].unwrap(), tt_move.unwrap()); // MainTT
    assert_eq!(
        move_vec[1].unwrap(),
        Move::new_unpromote(Square::SQ88, Square::SQ44, Piece::B_BISHOP)
    ); // GoodCapture
    assert_eq!(move_vec[2].unwrap(), killers[0].unwrap()); // Refutation
    assert_eq!(move_vec[3].unwrap(), killers[1].unwrap()); // Refutation
    assert_eq!(move_vec[4].unwrap(), cm.unwrap()); // Refutation
    assert!(move_vec[5..10]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ88, Square::SQ55, Piece::B_BISHOP))); // Quiet
    assert!(move_vec[5..10]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ89, Square::SQ78, Piece::B_KING))); // Quiet
    assert!(move_vec[5..10]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ89, Square::SQ79, Piece::B_KING))); // Quiet
    assert!(move_vec[5..10]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ89, Square::SQ98, Piece::B_KING))); // Quiet
    assert!(move_vec[5..10]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ89, Square::SQ99, Piece::B_KING))); // Quiet
    assert_eq!(
        move_vec[10].unwrap(),
        Move::new_unpromote(Square::SQ88, Square::SQ97, Piece::B_BISHOP)
    ); // BadCapture
    let m = mp.next_move(&pos, skip_quiets);
    assert!(m.is_none());
}

#[test]
fn test_move_picker_for_main_search_next_move_evasion() {
    let sfen = "k8/9/9/5b3/6K2/l8/p8/1B7/9 b - 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let tt_move = Some(Move::new_unpromote(Square::SQ35, Square::SQ24, Piece::B_KING));
    let mh = ButterflyHistory::new();
    let lph = LowPlyHistory::new();
    let cph = CapturePieceToHistory::new();
    let ch = [
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
    ];
    let ch = ch.iter().map(|x| x as *const PieceToHistory).collect::<Vec<_>>();
    let killers = [
        Some(Move::new_unpromote(Square::SQ88, Square::SQ99, Piece::B_BISHOP)),
        Some(Move::new_unpromote(Square::SQ88, Square::SQ79, Piece::B_BISHOP)),
    ];
    let cm = Some(Move::new_unpromote(Square::SQ88, Square::SQ77, Piece::B_BISHOP));
    let skip_quiets = false;
    let mut mp = MovePickerForMainSearch::new(&pos, tt_move, Depth(5), &mh, &lph, &cph, &ch, cm, &killers, 0);
    let moves_size;
    {
        let mut mlist = MoveList::new();
        mlist.generate::<LegalType>(&pos, 0);
        moves_size = mlist.size;
    }
    let mut move_vec = vec![];
    for _ in 0..moves_size {
        let m = mp.next_move(&pos, skip_quiets);
        move_vec.push(m);
    }
    assert_eq!(move_vec[0].unwrap(), tt_move.unwrap()); // EvasionTT
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ25, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ34, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ36, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ44, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ45, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ46, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ88, Square::SQ44, Piece::B_BISHOP))); // Evasion
    let m = mp.next_move(&pos, skip_quiets);
    assert!(m.is_none());
}

#[test]
fn test_move_picker_for_qsearch_next_move() {
    let sfen = "k8/9/9/5b3/9/l8/p8/1B7/1K7 b - 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let tt_move = Some(Move::new_unpromote(Square::SQ88, Square::SQ66, Piece::B_BISHOP));
    let mh = ButterflyHistory::new();
    let cph = CapturePieceToHistory::new();
    let ch = [
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
    ];
    let ch = ch.iter().map(|x| x as *const PieceToHistory).collect::<Vec<_>>();
    let recapture_square = Square::SQ97;
    let mut mp = MovePickerForQSearch::new(&mh, &cph, &ch, &pos, recapture_square, tt_move, Depth(0));
    let m = mp.next_move(&pos);
    assert_eq!(m.unwrap(), tt_move.unwrap()); // QSearchTT
    let m = mp.next_move(&pos);
    assert_eq!(m.unwrap(), Move::new_unpromote(Square::SQ88, Square::SQ44, Piece::B_BISHOP)); // Capture
    let m = mp.next_move(&pos);
    assert_eq!(m.unwrap(), Move::new_unpromote(Square::SQ88, Square::SQ97, Piece::B_BISHOP)); // Capture
    let m = mp.next_move(&pos);
    assert!(m.is_none());
}

#[test]
fn test_move_picker_for_qsearch_next_move_evasion() {
    let sfen = "k8/9/9/5b3/6K2/l8/p8/1B7/9 b - 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let tt_move = Some(Move::new_unpromote(Square::SQ35, Square::SQ24, Piece::B_KING));
    let mh = ButterflyHistory::new();
    let cph = CapturePieceToHistory::new();
    let ch = [
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
    ];
    let ch = ch.iter().map(|x| x as *const PieceToHistory).collect::<Vec<_>>();
    let recapture_square = Square::SQ97;
    let mut mp = MovePickerForQSearch::new(&mh, &cph, &ch, &pos, recapture_square, tt_move, Depth(0));
    let moves_size;
    {
        let mut mlist = MoveList::new();
        mlist.generate::<LegalType>(&pos, 0);
        moves_size = mlist.size;
    }
    let mut move_vec = vec![];
    for _ in 0..moves_size {
        let m = mp.next_move(&pos);
        move_vec.push(m);
    }
    assert_eq!(move_vec[0].unwrap(), tt_move.unwrap()); // EvasionTT
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ25, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ34, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ36, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ44, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ45, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ35, Square::SQ46, Piece::B_KING))); // Evasion
    assert!(move_vec[1..8]
        .iter()
        .any(|x| x.unwrap() == Move::new_unpromote(Square::SQ88, Square::SQ44, Piece::B_BISHOP))); // Evasion
    let m = mp.next_move(&pos);
    assert!(m.is_none());
}

#[test]
fn test_move_picker_for_qsearch_next_move_recapture() {
    let sfen = "k8/9/9/5b3/9/l8/p8/1B7/1K7 b - 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let tt_move = Some(Move::new_unpromote(Square::SQ88, Square::SQ66, Piece::B_BISHOP)); // to_square is not recapture_square. tt_move is not used.
    let mh = ButterflyHistory::new();
    let cph = CapturePieceToHistory::new();
    let ch = [
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
        PieceToHistory::new(),
    ];
    let ch = ch.iter().map(|x| x as *const PieceToHistory).collect::<Vec<_>>();
    let recapture_square = Square::SQ97;
    let mut mp = MovePickerForQSearch::new(&mh, &cph, &ch, &pos, recapture_square, tt_move, Depth::QS_RECAPTURES);
    let m = mp.next_move(&pos);
    assert_eq!(m.unwrap(), Move::new_unpromote(Square::SQ88, Square::SQ97, Piece::B_BISHOP)); // QRecapture
    let m = mp.next_move(&pos);
    assert!(m.is_none());
}

#[test]
fn test_move_picker_for_prob_cut_next_move() {
    let sfen = "k8/9/9/5b3/9/l8/p8/1B7/1K7 b - 1";
    let pos = Position::new_from_sfen(sfen).unwrap();
    let tt_move = Some(Move::new_unpromote(Square::SQ88, Square::SQ66, Piece::B_BISHOP));
    let cph = CapturePieceToHistory::new();
    let mut mp = MovePickerForProbCut::new(&pos, tt_move, Value(0), &cph);
    // ProbCut::TT uses tt_move if tt_move is good capture.
    // this tt_move is not used because it is capture move.
    let m = mp.next_move(&pos);
    assert_eq!(m.unwrap(), Move::new_unpromote(Square::SQ88, Square::SQ44, Piece::B_BISHOP)); // ProbCut

    // ProbCut uses only good capture moves.
    // A move from SQ88 to SQ97 is not good capture.
    let m = mp.next_move(&pos);
    assert!(m.is_none());
}

#[test]
fn test_pick_best() {
    let mut v = vec![
        ExtMove {
            mv: Move::new_unpromote(Square::SQ11, Square::SQ12, Piece::W_LANCE),
            score: 200,
        },
        ExtMove {
            mv: Move::new_unpromote(Square::SQ11, Square::SQ13, Piece::W_LANCE),
            score: 0,
        },
        ExtMove {
            mv: Move::new_unpromote(Square::SQ11, Square::SQ14, Piece::W_LANCE),
            score: 400,
        },
        ExtMove {
            mv: Move::new_unpromote(Square::SQ11, Square::SQ15, Piece::W_LANCE),
            score: 300,
        },
        ExtMove {
            mv: Move::new_unpromote(Square::SQ11, Square::SQ16, Piece::W_LANCE),
            score: 100,
        },
        ExtMove {
            mv: Move::new_unpromote(Square::SQ11, Square::SQ17, Piece::W_LANCE),
            score: 500,
        },
    ];
    let m = pick_best(&mut v);
    assert_eq!(m, Move::new_unpromote(Square::SQ11, Square::SQ17, Piece::W_LANCE));
    let m = pick_best(&mut v[1..]);
    assert_eq!(m, Move::new_unpromote(Square::SQ11, Square::SQ14, Piece::W_LANCE));
    let m = pick_best(&mut v[2..]);
    assert_eq!(m, Move::new_unpromote(Square::SQ11, Square::SQ15, Piece::W_LANCE));
    let m = pick_best(&mut v[3..]);
    assert_eq!(m, Move::new_unpromote(Square::SQ11, Square::SQ12, Piece::W_LANCE));
    let m = pick_best(&mut v[4..]);
    assert_eq!(m, Move::new_unpromote(Square::SQ11, Square::SQ16, Piece::W_LANCE));
    let m = pick_best(&mut v[5..]);
    assert_eq!(m, Move::new_unpromote(Square::SQ11, Square::SQ13, Piece::W_LANCE));
}
