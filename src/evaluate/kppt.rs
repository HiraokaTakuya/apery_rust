use crate::movetypes::*;
use crate::position::*;
use crate::search::*;
use crate::thread::*;
use crate::types::*;
use rayon::prelude::*;
use std::io::prelude::*;

pub const LIST_NUM: usize = 38; // Num of all pieces without 2 Kings.
const FV_SCALE: i32 = 32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EvalIndex(pub usize);

impl EvalIndex {
    pub const F_HAND_PAWN: EvalIndex = EvalIndex(0);
    pub const E_HAND_PAWN: EvalIndex = EvalIndex(EvalIndex::F_HAND_PAWN.0 + 19);
    pub const F_HAND_LANCE: EvalIndex = EvalIndex(EvalIndex::E_HAND_PAWN.0 + 19);
    pub const E_HAND_LANCE: EvalIndex = EvalIndex(EvalIndex::F_HAND_LANCE.0 + 5);
    pub const F_HAND_KNIGHT: EvalIndex = EvalIndex(EvalIndex::E_HAND_LANCE.0 + 5);
    pub const E_HAND_KNIGHT: EvalIndex = EvalIndex(EvalIndex::F_HAND_KNIGHT.0 + 5);
    pub const F_HAND_SILVER: EvalIndex = EvalIndex(EvalIndex::E_HAND_KNIGHT.0 + 5);
    pub const E_HAND_SILVER: EvalIndex = EvalIndex(EvalIndex::F_HAND_SILVER.0 + 5);
    pub const F_HAND_GOLD: EvalIndex = EvalIndex(EvalIndex::E_HAND_SILVER.0 + 5);
    pub const E_HAND_GOLD: EvalIndex = EvalIndex(EvalIndex::F_HAND_GOLD.0 + 5);
    pub const F_HAND_BISHOP: EvalIndex = EvalIndex(EvalIndex::E_HAND_GOLD.0 + 5);
    pub const E_HAND_BISHOP: EvalIndex = EvalIndex(EvalIndex::F_HAND_BISHOP.0 + 3);
    pub const F_HAND_ROOK: EvalIndex = EvalIndex(EvalIndex::E_HAND_BISHOP.0 + 3);
    pub const E_HAND_ROOK: EvalIndex = EvalIndex(EvalIndex::F_HAND_ROOK.0 + 3);
    pub const FE_HAND_END: EvalIndex = EvalIndex(EvalIndex::E_HAND_ROOK.0 + 3);

    pub const F_PAWN: EvalIndex = EvalIndex(EvalIndex::FE_HAND_END.0);
    pub const E_PAWN: EvalIndex = EvalIndex(EvalIndex::F_PAWN.0 + 81);
    pub const F_LANCE: EvalIndex = EvalIndex(EvalIndex::E_PAWN.0 + 81);
    pub const E_LANCE: EvalIndex = EvalIndex(EvalIndex::F_LANCE.0 + 81);
    pub const F_KNIGHT: EvalIndex = EvalIndex(EvalIndex::E_LANCE.0 + 81);
    pub const E_KNIGHT: EvalIndex = EvalIndex(EvalIndex::F_KNIGHT.0 + 81);
    pub const F_SILVER: EvalIndex = EvalIndex(EvalIndex::E_KNIGHT.0 + 81);
    pub const E_SILVER: EvalIndex = EvalIndex(EvalIndex::F_SILVER.0 + 81);
    pub const F_GOLD: EvalIndex = EvalIndex(EvalIndex::E_SILVER.0 + 81);
    pub const E_GOLD: EvalIndex = EvalIndex(EvalIndex::F_GOLD.0 + 81);
    pub const F_BISHOP: EvalIndex = EvalIndex(EvalIndex::E_GOLD.0 + 81);
    pub const E_BISHOP: EvalIndex = EvalIndex(EvalIndex::F_BISHOP.0 + 81);
    pub const F_HORSE: EvalIndex = EvalIndex(EvalIndex::E_BISHOP.0 + 81);
    pub const E_HORSE: EvalIndex = EvalIndex(EvalIndex::F_HORSE.0 + 81);
    pub const F_ROOK: EvalIndex = EvalIndex(EvalIndex::E_HORSE.0 + 81);
    pub const E_ROOK: EvalIndex = EvalIndex(EvalIndex::F_ROOK.0 + 81);
    pub const F_DRAGON: EvalIndex = EvalIndex(EvalIndex::E_ROOK.0 + 81);
    pub const E_DRAGON: EvalIndex = EvalIndex(EvalIndex::F_DRAGON.0 + 81);
    pub const FE_END: EvalIndex = EvalIndex(EvalIndex::E_DRAGON.0 + 81);

    const TABLE_OF_EVAL_INDEX_NEW_BOARD: [EvalIndex; Piece::NUM] = [
        EvalIndex(0), // Piece::EMPTY
        EvalIndex::F_PAWN,
        EvalIndex::F_LANCE,
        EvalIndex::F_KNIGHT,
        EvalIndex::F_SILVER,
        EvalIndex::F_BISHOP,
        EvalIndex::F_ROOK,
        EvalIndex::F_GOLD,
        EvalIndex(0), // Piece::B_KING
        EvalIndex::F_GOLD,
        EvalIndex::F_GOLD,
        EvalIndex::F_GOLD,
        EvalIndex::F_GOLD,
        EvalIndex::F_HORSE,
        EvalIndex::F_DRAGON,
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex::E_PAWN,
        EvalIndex::E_LANCE,
        EvalIndex::E_KNIGHT,
        EvalIndex::E_SILVER,
        EvalIndex::E_BISHOP,
        EvalIndex::E_ROOK,
        EvalIndex::E_GOLD,
        EvalIndex(0), // Piece::W_KING
        EvalIndex::E_GOLD,
        EvalIndex::E_GOLD,
        EvalIndex::E_GOLD,
        EvalIndex::E_GOLD,
        EvalIndex::E_HORSE,
        EvalIndex::E_DRAGON,
    ];
    const TABLE_OF_EVAL_INDEX_NEW_HAND: [EvalIndex; Piece::NUM] = [
        EvalIndex(0),
        EvalIndex::F_HAND_PAWN,
        EvalIndex::F_HAND_LANCE,
        EvalIndex::F_HAND_KNIGHT,
        EvalIndex::F_HAND_SILVER,
        EvalIndex::F_HAND_BISHOP,
        EvalIndex::F_HAND_ROOK,
        EvalIndex::F_HAND_GOLD,
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex::E_HAND_PAWN,
        EvalIndex::E_HAND_LANCE,
        EvalIndex::E_HAND_KNIGHT,
        EvalIndex::E_HAND_SILVER,
        EvalIndex::E_HAND_BISHOP,
        EvalIndex::E_HAND_ROOK,
        EvalIndex::E_HAND_GOLD,
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
        EvalIndex(0),
    ];
    pub fn new_board(pc: Piece) -> EvalIndex {
        debug_assert!(0 <= pc.0);
        debug_assert!((pc.0 as usize) < Piece::NUM);
        unsafe { *EvalIndex::TABLE_OF_EVAL_INDEX_NEW_BOARD.get_unchecked(pc.0 as usize) }
    }
    pub fn new_hand(pc: Piece) -> EvalIndex {
        debug_assert!(0 <= pc.0);
        debug_assert!((pc.0 as usize) < Piece::NUM);
        unsafe { *EvalIndex::TABLE_OF_EVAL_INDEX_NEW_HAND.get_unchecked(pc.0 as usize) }
    }
    pub fn inverse(self) -> EvalIndex {
        unsafe { *INVERSE_EVAL_INDEX_TABLE.get_unchecked(self.0) }
    }
}

static INVERSE_EVAL_INDEX_TABLE: once_cell::sync::Lazy<[EvalIndex; EvalIndex::FE_END.0]> = once_cell::sync::Lazy::new(|| {
    let mut buf = [EvalIndex(0); EvalIndex::FE_END.0];
    for (index, item) in buf.iter_mut().enumerate() {
        let eval_index = EvalIndex(index);
        if eval_index.0 < EvalIndex::E_HAND_PAWN.0 {
            *item = EvalIndex(eval_index.0 + 19);
        } else if eval_index.0 < EvalIndex::F_HAND_LANCE.0 {
            *item = EvalIndex(eval_index.0 - 19);
        } else if eval_index.0 < EvalIndex::E_HAND_LANCE.0 {
            *item = EvalIndex(eval_index.0 + 5);
        } else if eval_index.0 < EvalIndex::F_HAND_KNIGHT.0 {
            *item = EvalIndex(eval_index.0 - 5);
        } else if eval_index.0 < EvalIndex::E_HAND_KNIGHT.0 {
            *item = EvalIndex(eval_index.0 + 5);
        } else if eval_index.0 < EvalIndex::F_HAND_SILVER.0 {
            *item = EvalIndex(eval_index.0 - 5);
        } else if eval_index.0 < EvalIndex::E_HAND_SILVER.0 {
            *item = EvalIndex(eval_index.0 + 5);
        } else if eval_index.0 < EvalIndex::F_HAND_GOLD.0 {
            *item = EvalIndex(eval_index.0 - 5);
        } else if eval_index.0 < EvalIndex::E_HAND_GOLD.0 {
            *item = EvalIndex(eval_index.0 + 5);
        } else if eval_index.0 < EvalIndex::F_HAND_BISHOP.0 {
            *item = EvalIndex(eval_index.0 - 5);
        } else if eval_index.0 < EvalIndex::E_HAND_BISHOP.0 {
            *item = EvalIndex(eval_index.0 + 3);
        } else if eval_index.0 < EvalIndex::F_HAND_ROOK.0 {
            *item = EvalIndex(eval_index.0 - 3);
        } else if eval_index.0 < EvalIndex::E_HAND_ROOK.0 {
            *item = EvalIndex(eval_index.0 + 3);
        } else if eval_index.0 < EvalIndex::F_PAWN.0 {
            *item = EvalIndex(eval_index.0 - 3);
        } else if eval_index.0 < EvalIndex::E_PAWN.0 {
            let sq = Square((eval_index.0 - EvalIndex::F_PAWN.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize + 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::F_LANCE.0 {
            let sq = Square((eval_index.0 - EvalIndex::E_PAWN.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize - 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::E_LANCE.0 {
            let sq = Square((eval_index.0 - EvalIndex::F_LANCE.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize + 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::F_KNIGHT.0 {
            let sq = Square((eval_index.0 - EvalIndex::E_LANCE.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize - 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::E_KNIGHT.0 {
            let sq = Square((eval_index.0 - EvalIndex::F_KNIGHT.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize + 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::F_SILVER.0 {
            let sq = Square((eval_index.0 - EvalIndex::E_KNIGHT.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize - 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::E_SILVER.0 {
            let sq = Square((eval_index.0 - EvalIndex::F_SILVER.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize + 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::F_GOLD.0 {
            let sq = Square((eval_index.0 - EvalIndex::E_SILVER.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize - 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::E_GOLD.0 {
            let sq = Square((eval_index.0 - EvalIndex::F_GOLD.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize + 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::F_BISHOP.0 {
            let sq = Square((eval_index.0 - EvalIndex::E_GOLD.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize - 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::E_BISHOP.0 {
            let sq = Square((eval_index.0 - EvalIndex::F_BISHOP.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize + 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::F_HORSE.0 {
            let sq = Square((eval_index.0 - EvalIndex::E_BISHOP.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize - 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::E_HORSE.0 {
            let sq = Square((eval_index.0 - EvalIndex::F_HORSE.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize + 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::F_ROOK.0 {
            let sq = Square((eval_index.0 - EvalIndex::E_HORSE.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize - 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::E_ROOK.0 {
            let sq = Square((eval_index.0 - EvalIndex::F_ROOK.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize + 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::F_DRAGON.0 {
            let sq = Square((eval_index.0 - EvalIndex::E_ROOK.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize - 81 + sq.inverse().0 as usize);
        } else if eval_index.0 < EvalIndex::E_DRAGON.0 {
            let sq = Square((eval_index.0 - EvalIndex::F_DRAGON.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize + 81 + sq.inverse().0 as usize);
        } else {
            let sq = Square((eval_index.0 - EvalIndex::E_DRAGON.0) as i32);
            *item = EvalIndex(eval_index.0 - sq.0 as usize - 81 + sq.inverse().0 as usize);
        }
    }
    buf
});

pub struct Evaluator {
    pub kpp: *const [[[[i16; 2]; EvalIndex::FE_END.0]; EvalIndex::FE_END.0]; Square::NUM],
    pub kkp: *const [[[[i16; 2]; EvalIndex::FE_END.0]; Square::NUM]; Square::NUM],
}

impl Evaluator {
    fn load_kpp(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::open(path)?;
        let ptr = BUFFER_KPP.lock().unwrap().as_mut_ptr() as *mut u8;
        let slice =
            unsafe { std::slice::from_raw_parts_mut(ptr, 2 * 2 * EvalIndex::FE_END.0 * EvalIndex::FE_END.0 * Square::NUM) };
        file.read_exact(slice)?;
        self.kpp =
            BUFFER_KPP.lock().unwrap().as_mut_ptr() as *mut [[[[i16; 2]; EvalIndex::FE_END.0]; EvalIndex::FE_END.0]; Square::NUM];
        Ok(())
    }
    fn load_kkp(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::open(path)?;
        let ptr = BUFFER_KKP.lock().unwrap().as_mut_ptr() as *mut u8;
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr, 2 * 2 * EvalIndex::FE_END.0 * Square::NUM * Square::NUM) };
        file.read_exact(slice)?;
        self.kkp = BUFFER_KKP.lock().unwrap().as_mut_ptr() as *mut [[[[i16; 2]; EvalIndex::FE_END.0]; Square::NUM]; Square::NUM];
        Ok(())
    }
    fn write_kpp(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        let slice: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.kpp as *const u8,
                2 * 2 * EvalIndex::FE_END.0 * EvalIndex::FE_END.0 * Square::NUM,
            )
        };
        file.write_all(slice)?;
        Ok(())
    }
    fn write_kkp(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        let slice: &[u8] =
            unsafe { std::slice::from_raw_parts(self.kkp as *const u8, 2 * 2 * EvalIndex::FE_END.0 * Square::NUM * Square::NUM) };
        file.write_all(slice)?;
        Ok(())
    }
    #[inline]
    pub fn kpp(&self, sq: Square, i: EvalIndex, j: EvalIndex) -> [i16; 2] {
        unsafe { *(*self.kpp).get_unchecked(sq.0 as usize).get_unchecked(i.0).get_unchecked(j.0) }
    }
    #[inline]
    pub fn kkp(&self, sq0: Square, sq1: Square, i: EvalIndex) -> [i16; 2] {
        unsafe {
            *(*self.kkp)
                .get_unchecked(sq0.0 as usize)
                .get_unchecked(sq1.0 as usize)
                .get_unchecked(i.0)
        }
    }
    fn evaluate_at_root(&self, pos: &Position, stack: &mut [Stack]) -> Value {
        let sq_bk = pos.king_square(Color::BLACK);
        let sq_wk = pos.king_square(Color::WHITE);
        let sq_wk_inv = sq_wk.inverse();
        let list = pos.eval_list();
        let sum = &mut get_stack_mut(stack, 0).static_eval_raw;
        *sum = EvalSum::new();
        for i in 0..LIST_NUM {
            let k0 = list.get(i, Color::BLACK);
            let k1 = list.get(i, Color::WHITE);
            for j in 0..i {
                let l0 = list.get(j, Color::BLACK);
                let l1 = list.get(j, Color::WHITE);
                let board_and_turn_0 = self.kpp(sq_bk, k0, l0);
                sum.val[0][0] += i32::from(board_and_turn_0[0]);
                sum.val[0][1] += i32::from(board_and_turn_0[1]);
                let board_and_turn_1 = self.kpp(sq_wk_inv, k1, l1);
                sum.val[1][0] += i32::from(board_and_turn_1[0]);
                sum.val[1][1] += i32::from(board_and_turn_1[1]);
            }
            let board_and_turn = self.kkp(sq_bk, sq_wk, k0);
            sum.val[2][0] += i32::from(board_and_turn[0]);
            sum.val[2][1] += i32::from(board_and_turn[1]);
        }
        sum.val[2][0] += pos.material().0 * FV_SCALE;
        sum.sum(pos.side_to_move()) / FV_SCALE
    }
    fn doapc(&self, pos: &Position, eval_index: EvalIndex) -> EvalSum {
        let sq_bk = pos.king_square(Color::BLACK);
        let sq_wk = pos.king_square(Color::WHITE);
        let mut eval_sum = EvalSum::new();
        let board_and_turn = self.kkp(sq_bk, sq_wk, eval_index);
        eval_sum.val[2][0] = i32::from(board_and_turn[0]);
        eval_sum.val[2][1] = i32::from(board_and_turn[1]);
        let inv_sq_wk = sq_wk.inverse();
        let inv_eval_index = eval_index.inverse();
        for item in pos.eval_list().0.iter() {
            let board_and_turn_0 = self.kpp(sq_bk, eval_index, item[0]);
            eval_sum.val[0][0] += i32::from(board_and_turn_0[0]);
            eval_sum.val[0][1] += i32::from(board_and_turn_0[1]);
            let board_and_turn_1 = self.kpp(inv_sq_wk, inv_eval_index, item[1]);
            eval_sum.val[1][0] += i32::from(board_and_turn_1[0]);
            eval_sum.val[1][1] += i32::from(board_and_turn_1[1]);
        }
        eval_sum
    }
    fn doablack(&self, pos: &Position, eval_index: EvalIndex) -> [i32; 2] {
        let sq_bk = pos.king_square(Color::BLACK);
        let mut sum = [0, 0];
        for item in pos.eval_list().0.iter() {
            let board_and_turn = self.kpp(sq_bk, eval_index, item[0]);
            sum[0] += i32::from(board_and_turn[0]);
            sum[1] += i32::from(board_and_turn[1]);
        }
        sum
    }
    fn doawhite(&self, pos: &Position, inv_eval_index: EvalIndex) -> [i32; 2] {
        let inv_sq_wk = pos.king_square(Color::WHITE).inverse();
        let mut sum = [0, 0];
        for item in pos.eval_list().0.iter() {
            let board_and_turn = self.kpp(inv_sq_wk, inv_eval_index, item[1]);
            sum[0] += i32::from(board_and_turn[0]);
            sum[1] += i32::from(board_and_turn[1]);
        }
        sum
    }
    fn evaluate_difference_calc(&self, pos: &mut Position, stack: &mut [Stack], ehash: *mut EvalHash) -> Value {
        if get_stack(stack, 0).static_eval_raw.is_not_evaluated() {
            debug_assert!(!get_stack(stack, -1).static_eval_raw.is_not_evaluated());
            debug_assert!(get_stack(stack, -1).current_move.non_zero_unwrap_unchecked() != Move::NULL);
            let key_excluded_turn = pos.key().excluded_turn();
            let mut entry = unsafe { (*ehash).get(key_excluded_turn) };
            entry.decode();
            if entry.key == key_excluded_turn {
                get_stack_mut(stack, 0).static_eval_raw = entry;
                debug_assert_eq!(entry.sum(pos.side_to_move()), self.evaluate_debug(pos));
                return entry.sum(pos.side_to_move()) / FV_SCALE;
            }

            let last_move = get_stack(stack, -1).current_move.non_zero_unwrap_unchecked();
            let sq_bk = pos.king_square(Color::BLACK);
            let sq_wk = pos.king_square(Color::WHITE);
            if PieceType::new(last_move.piece_moved_before_move()) == PieceType::KING {
                get_stack_mut(stack, 0).static_eval_raw = get_stack(stack, -1).static_eval_raw;
                let sum = &mut get_stack_mut(stack, 0).static_eval_raw;
                sum.val[2][0] = pos.material().0 * FV_SCALE;
                sum.val[2][1] = 0;
                if pos.side_to_move() == Color::BLACK {
                    sum.val[1][0] = 0;
                    sum.val[1][1] = 0;
                    let inv_sq_wk = sq_wk.inverse();
                    let eval_list = pos.eval_list();
                    for (i, item) in eval_list.0.iter().enumerate() {
                        for item_inner in eval_list.0.iter().take(i) {
                            let board_and_turn = self.kpp(inv_sq_wk, item[1], item_inner[1]);
                            sum.val[1][0] += i32::from(board_and_turn[0]);
                            sum.val[1][1] += i32::from(board_and_turn[1]);
                        }
                        let board_and_turn = self.kkp(inv_sq_wk, sq_bk.inverse(), item[1]);
                        sum.val[2][0] -= i32::from(board_and_turn[0]);
                        sum.val[2][1] += i32::from(board_and_turn[1]);
                    }

                    if pos.is_capture_after_move() {
                        let changed_eval_index_captured = pos.changed_eval_index_captured();
                        let diff = self.doablack(pos, changed_eval_index_captured.new_index);
                        sum.val[0][0] += diff[0];
                        sum.val[0][1] += diff[1];
                        let list_index_captured = pos.eval_list_index(changed_eval_index_captured.new_index);
                        pos.eval_list_mut()
                            .set(list_index_captured, Color::BLACK, changed_eval_index_captured.old_index);
                        let diff = self.doablack(pos, changed_eval_index_captured.old_index);
                        sum.val[0][0] -= diff[0];
                        sum.val[0][1] -= diff[1];
                        pos.eval_list_mut()
                            .set(list_index_captured, Color::BLACK, changed_eval_index_captured.new_index);
                    }
                } else {
                    sum.val[0][0] = 0;
                    sum.val[0][1] = 0;
                    let eval_list = pos.eval_list();
                    for (i, item) in eval_list.0.iter().enumerate() {
                        for item_inner in eval_list.0.iter().take(i) {
                            let board_and_turn = self.kpp(sq_bk, item[0], item_inner[0]);
                            sum.val[0][0] += i32::from(board_and_turn[0]);
                            sum.val[0][1] += i32::from(board_and_turn[1]);
                        }
                        let board_and_turn = self.kkp(sq_bk, sq_wk, item[0]);
                        sum.val[2][0] += i32::from(board_and_turn[0]);
                        sum.val[2][1] += i32::from(board_and_turn[1]);
                    }

                    if pos.is_capture_after_move() {
                        let changed_eval_index_captured = pos.changed_eval_index_captured();
                        let diff = self.doawhite(pos, changed_eval_index_captured.new_index.inverse());
                        sum.val[1][0] += diff[0];
                        sum.val[1][1] += diff[1];
                        let list_index_captured = pos.eval_list_index(changed_eval_index_captured.new_index);
                        pos.eval_list_mut().set(
                            list_index_captured,
                            Color::WHITE,
                            changed_eval_index_captured.old_index.inverse(),
                        );
                        let diff = self.doawhite(pos, changed_eval_index_captured.old_index.inverse());
                        sum.val[1][0] -= diff[0];
                        sum.val[1][1] -= diff[1];
                        pos.eval_list_mut().set(
                            list_index_captured,
                            Color::WHITE,
                            changed_eval_index_captured.new_index.inverse(),
                        );
                    }
                }
                sum.key = key_excluded_turn;
                sum.encode();
                debug_assert_eq!(sum.sum(pos.side_to_move()), self.evaluate_debug(pos));
                unsafe {
                    (*ehash).set(key_excluded_turn, sum);
                }
                sum.sum(pos.side_to_move()) / FV_SCALE
            } else {
                let list_index = pos.eval_list_index(pos.changed_eval_index().old_index);
                let mut diff = self.doapc(pos, pos.changed_eval_index().new_index);
                if pos.is_capture_after_move() {
                    let inv_sq_wk = sq_wk.inverse();
                    diff += self.doapc(pos, pos.changed_eval_index_captured().new_index);
                    let board_and_turn = self.kpp(
                        sq_bk,
                        pos.changed_eval_index().new_index,
                        pos.changed_eval_index_captured().new_index,
                    );
                    diff.val[0][0] -= i32::from(board_and_turn[0]);
                    diff.val[0][1] -= i32::from(board_and_turn[1]);
                    let board_and_turn = self.kpp(
                        inv_sq_wk,
                        pos.changed_eval_index().new_index.inverse(),
                        pos.changed_eval_index_captured().new_index.inverse(),
                    );
                    diff.val[1][0] -= i32::from(board_and_turn[0]);
                    diff.val[1][1] -= i32::from(board_and_turn[1]);
                    let changed_eval_index_captured = pos.changed_eval_index_captured();
                    let list_index_captured = pos.eval_list_index(changed_eval_index_captured.new_index);
                    pos.eval_list_mut()
                        .set(list_index_captured, Color::BLACK, changed_eval_index_captured.old_index);
                    pos.eval_list_mut().set(
                        list_index_captured,
                        Color::WHITE,
                        changed_eval_index_captured.old_index.inverse(),
                    );

                    let changed_eval_index = pos.changed_eval_index();
                    pos.eval_list_mut()
                        .set(list_index, Color::BLACK, changed_eval_index.old_index);
                    pos.eval_list_mut()
                        .set(list_index, Color::WHITE, changed_eval_index.old_index.inverse());
                    diff -= self.doapc(pos, pos.changed_eval_index().old_index);
                    diff -= self.doapc(pos, pos.changed_eval_index_captured().old_index);

                    let board_and_turn = self.kpp(
                        sq_bk,
                        pos.changed_eval_index().old_index,
                        pos.changed_eval_index_captured().old_index,
                    );
                    diff.val[0][0] += i32::from(board_and_turn[0]);
                    diff.val[0][1] += i32::from(board_and_turn[1]);
                    let board_and_turn = self.kpp(
                        inv_sq_wk,
                        pos.changed_eval_index().old_index.inverse(),
                        pos.changed_eval_index_captured().old_index.inverse(),
                    );
                    diff.val[1][0] += i32::from(board_and_turn[0]);
                    diff.val[1][1] += i32::from(board_and_turn[1]);

                    let changed_eval_index_captured = pos.changed_eval_index_captured();
                    pos.eval_list_mut()
                        .set(list_index_captured, Color::BLACK, changed_eval_index_captured.new_index);
                    pos.eval_list_mut().set(
                        list_index_captured,
                        Color::WHITE,
                        changed_eval_index_captured.new_index.inverse(),
                    );
                } else {
                    let old_index = pos.changed_eval_index().old_index;
                    pos.eval_list_mut().set(list_index, Color::BLACK, old_index);
                    pos.eval_list_mut().set(list_index, Color::WHITE, old_index.inverse());
                    diff -= self.doapc(pos, old_index);
                }
                let changed_eval_index = pos.changed_eval_index();
                pos.eval_list_mut()
                    .set(list_index, Color::BLACK, changed_eval_index.new_index);
                pos.eval_list_mut()
                    .set(list_index, Color::WHITE, changed_eval_index.new_index.inverse());
                diff.val[2][0] += pos.material_diff().0 * FV_SCALE;
                get_stack_mut(stack, 0).static_eval_raw = get_stack(stack, -1).static_eval_raw;
                get_stack_mut(stack, 0).static_eval_raw += diff;
                get_stack_mut(stack, 0).static_eval_raw.key = key_excluded_turn;
                get_stack_mut(stack, 0).static_eval_raw.encode();
                debug_assert_eq!(
                    get_stack(stack, 0).static_eval_raw.sum(pos.side_to_move()),
                    self.evaluate_debug(pos)
                );
                get_stack(stack, 0).static_eval_raw.sum(pos.side_to_move()) / FV_SCALE
            }
        } else {
            debug_assert_eq!(
                get_stack(stack, 0).static_eval_raw.sum(pos.side_to_move()),
                self.evaluate_debug(pos)
            );
            get_stack(stack, 0).static_eval_raw.sum(pos.side_to_move()) / FV_SCALE
        }
    }
    #[allow(dead_code)]
    fn evaluate_debug(&self, pos: &Position) -> Value {
        let sq_bk = pos.king_square(Color::BLACK);
        let sq_wk = pos.king_square(Color::WHITE);
        let sq_wk_inv = sq_wk.inverse();
        let list = EvalList::new(&pos.base);
        let mut sum = EvalSum::new();
        for i in 0..LIST_NUM {
            let k0 = list.get(i, Color::BLACK);
            let k1 = list.get(i, Color::WHITE);
            for j in 0..i {
                let l0 = list.get(j, Color::BLACK);
                let l1 = list.get(j, Color::WHITE);
                let board_and_turn_0 = self.kpp(sq_bk, k0, l0);
                sum.val[0][0] += i32::from(board_and_turn_0[0]);
                sum.val[0][1] += i32::from(board_and_turn_0[1]);
                let board_and_turn_1 = self.kpp(sq_wk_inv, k1, l1);
                sum.val[1][0] += i32::from(board_and_turn_1[0]);
                sum.val[1][1] += i32::from(board_and_turn_1[1]);
            }
            let board_and_turn = self.kkp(sq_bk, sq_wk, k0);
            sum.val[2][0] += i32::from(board_and_turn[0]);
            sum.val[2][1] += i32::from(board_and_turn[1]);
        }
        sum.val[2][0] += pos.material().0 * FV_SCALE;
        sum.sum(pos.side_to_move()) // not div by FV_SCALE
    }
}

static BUFFER_KPP: once_cell::sync::Lazy<std::sync::Mutex<Vec<i16>>> = once_cell::sync::Lazy::new(|| {
    std::sync::Mutex::new(Vec::<i16>::with_capacity(
        2 * EvalIndex::FE_END.0 * EvalIndex::FE_END.0 * Square::NUM,
    ))
});
static BUFFER_KKP: once_cell::sync::Lazy<std::sync::Mutex<Vec<i16>>> = once_cell::sync::Lazy::new(|| {
    std::sync::Mutex::new(Vec::<i16>::with_capacity(2 * EvalIndex::FE_END.0 * Square::NUM * Square::NUM))
});

pub static mut EVALUATOR: Evaluator = Evaluator {
    kpp: std::ptr::null(),
    kkp: std::ptr::null(),
};

pub fn load_evaluate_files(eval_dir: &str) -> Result<(), String> {
    let kpp_file_name = {
        let mut path = std::path::PathBuf::from(eval_dir);
        path.push("KPP.bin");
        path.as_path().as_os_str().to_str().unwrap().to_string()
    };
    if let Err(err) = unsafe { EVALUATOR.load_kpp(&kpp_file_name) } {
        return Err(format!("{}\nFile name: {}", err, kpp_file_name));
    }
    let kkp_file_name = {
        let mut path = std::path::PathBuf::from(eval_dir);
        path.push("KKP.bin");
        path.as_path().as_os_str().to_str().unwrap().to_string()
    };
    if let Err(err) = unsafe { EVALUATOR.load_kkp(&kkp_file_name) } {
        return Err(format!("{}\nFile name: {}", err, kkp_file_name));
    }
    Ok(())
}

pub fn write_evaluate_files() -> Result<(), String> {
    let kpp_file_name = "KPP.bin";
    let kkp_file_name = "KKP.bin";
    if let Err(err) = unsafe { EVALUATOR.write_kpp(kpp_file_name) } {
        return Err(format!("{}\nFile name: {}", err, kpp_file_name));
    }
    if let Err(err) = unsafe { EVALUATOR.write_kkp(kkp_file_name) } {
        return Err(format!("{}\nFile name: {}", err, kkp_file_name));
    }
    Ok(())
}

pub fn evaluate(pos: &mut Position, stack: &mut [Stack], ehash: *mut EvalHash) -> Value {
    unsafe { EVALUATOR.evaluate_difference_calc(pos, stack, ehash) }
}

pub fn evaluate_at_root(pos: &Position, stack: &mut [Stack]) -> Value {
    unsafe { EVALUATOR.evaluate_at_root(pos, stack) }
}

#[repr(align(256))]
#[derive(Clone, Copy)]
pub struct EvalSum {
    pub val: [[i32; 2]; 3],
    pub key: KeyExcludedTurn,
}

impl std::ops::AddAssign for EvalSum {
    fn add_assign(&mut self, other: EvalSum) {
        self.val[0][0] += other.val[0][0];
        self.val[0][1] += other.val[0][1];
        self.val[1][0] += other.val[1][0];
        self.val[1][1] += other.val[1][1];
        self.val[2][0] += other.val[2][0];
        self.val[2][1] += other.val[2][1];
    }
}

impl std::ops::SubAssign for EvalSum {
    fn sub_assign(&mut self, other: EvalSum) {
        self.val[0][0] -= other.val[0][0];
        self.val[0][1] -= other.val[0][1];
        self.val[1][0] -= other.val[1][0];
        self.val[1][1] -= other.val[1][1];
        self.val[2][0] -= other.val[2][0];
        self.val[2][1] -= other.val[2][1];
    }
}

impl EvalSum {
    const NOT_EVALUATED: i32 = i32::max_value();
    pub fn new() -> EvalSum {
        EvalSum {
            val: [[0; 2]; 3],
            key: KeyExcludedTurn(0),
        }
    }
    fn sum(&self, side_to_move: Color) -> Value {
        let value_board = {
            let pseudo_value_board = self.val[0][0] - self.val[1][0] + self.val[2][0];
            if side_to_move == Color::BLACK {
                pseudo_value_board
            } else {
                -pseudo_value_board
            }
        };
        let value_turn = self.val[0][1] + self.val[1][1] + self.val[2][1];
        Value(value_board + value_turn)
    }
    fn decode(&mut self) {
        self.key = KeyExcludedTurn(unsafe {
            self.key.0
                ^ std::mem::transmute::<[i32; 2], u64>(self.val[0])
                ^ std::mem::transmute::<[i32; 2], u64>(self.val[1])
                ^ std::mem::transmute::<[i32; 2], u64>(self.val[2])
        });
    }
    fn encode(&mut self) {
        self.decode();
    }
    pub fn set_not_evaluated(&mut self) {
        self.val[0][0] = EvalSum::NOT_EVALUATED;
    }
    pub fn is_not_evaluated(&self) -> bool {
        self.val[0][0] == EvalSum::NOT_EVALUATED
    }
}

#[derive(Clone)]
pub struct ChangedEvalIndex {
    pub new_index: EvalIndex,
    pub old_index: EvalIndex,
}

impl ChangedEvalIndex {
    pub const ZERO: ChangedEvalIndex = ChangedEvalIndex {
        new_index: EvalIndex(0),
        old_index: EvalIndex(0),
    };
}

pub struct EvalHash {
    value: Vec<EvalSum>,
}

impl EvalHash {
    pub fn new() -> EvalHash {
        EvalHash { value: vec![] }
    }
    // parallel zero clearing.
    pub fn clear(&mut self) {
        self.value.par_iter_mut().for_each(|x| {
            *x = unsafe { std::mem::zeroed() };
        });
    }
    pub fn get(&self, key: KeyExcludedTurn) -> EvalSum {
        let mask = self.value.len() - 1;
        let index = key.0 as usize & mask;
        unsafe { *self.value.get_unchecked(index) }
    }
    pub fn set(&mut self, key: KeyExcludedTurn, entry: &EvalSum) {
        let mask = self.value.len() - 1;
        let index = key.0 as usize & mask;
        unsafe {
            *self.value.get_unchecked_mut(index) = *entry;
        }
    }
    pub fn resize(&mut self, mega_byte_size: usize, thread_pool: &mut ThreadPool) {
        thread_pool.wait_for_search_finished();
        let mega_byte_size = (mega_byte_size + 1).next_power_of_two() >> 1;
        let len = mega_byte_size * 1024 * 1024 / std::mem::size_of::<EvalSum>();
        self.value.clear();
        self.value.shrink_to_fit();
        self.value = Vec::<EvalSum>::with_capacity(len);
        unsafe {
            self.value.set_len(len);
        }
        self.clear();
    }
}

#[test]
fn test_eval_index_new() {
    assert_eq!(EvalIndex::F_PAWN, EvalIndex::new_board(Piece::B_PAWN));
    assert_eq!(EvalIndex::F_LANCE, EvalIndex::new_board(Piece::B_LANCE));
    assert_eq!(EvalIndex::F_KNIGHT, EvalIndex::new_board(Piece::B_KNIGHT));
    assert_eq!(EvalIndex::F_SILVER, EvalIndex::new_board(Piece::B_SILVER));
    assert_eq!(EvalIndex::F_BISHOP, EvalIndex::new_board(Piece::B_BISHOP));
    assert_eq!(EvalIndex::F_ROOK, EvalIndex::new_board(Piece::B_ROOK));
    assert_eq!(EvalIndex::F_GOLD, EvalIndex::new_board(Piece::B_GOLD));
    assert_eq!(EvalIndex::F_GOLD, EvalIndex::new_board(Piece::B_PRO_PAWN));
    assert_eq!(EvalIndex::F_GOLD, EvalIndex::new_board(Piece::B_PRO_LANCE));
    assert_eq!(EvalIndex::F_GOLD, EvalIndex::new_board(Piece::B_PRO_KNIGHT));
    assert_eq!(EvalIndex::F_GOLD, EvalIndex::new_board(Piece::B_PRO_SILVER));
    assert_eq!(EvalIndex::F_HORSE, EvalIndex::new_board(Piece::B_HORSE));
    assert_eq!(EvalIndex::F_DRAGON, EvalIndex::new_board(Piece::B_DRAGON));
    assert_eq!(EvalIndex::E_PAWN, EvalIndex::new_board(Piece::W_PAWN));
    assert_eq!(EvalIndex::E_LANCE, EvalIndex::new_board(Piece::W_LANCE));
    assert_eq!(EvalIndex::E_KNIGHT, EvalIndex::new_board(Piece::W_KNIGHT));
    assert_eq!(EvalIndex::E_SILVER, EvalIndex::new_board(Piece::W_SILVER));
    assert_eq!(EvalIndex::E_BISHOP, EvalIndex::new_board(Piece::W_BISHOP));
    assert_eq!(EvalIndex::E_ROOK, EvalIndex::new_board(Piece::W_ROOK));
    assert_eq!(EvalIndex::E_GOLD, EvalIndex::new_board(Piece::W_GOLD));
    assert_eq!(EvalIndex::E_GOLD, EvalIndex::new_board(Piece::W_PRO_PAWN));
    assert_eq!(EvalIndex::E_GOLD, EvalIndex::new_board(Piece::W_PRO_LANCE));
    assert_eq!(EvalIndex::E_GOLD, EvalIndex::new_board(Piece::W_PRO_KNIGHT));
    assert_eq!(EvalIndex::E_GOLD, EvalIndex::new_board(Piece::W_PRO_SILVER));
    assert_eq!(EvalIndex::E_HORSE, EvalIndex::new_board(Piece::W_HORSE));
    assert_eq!(EvalIndex::E_DRAGON, EvalIndex::new_board(Piece::W_DRAGON));

    assert_eq!(EvalIndex::F_HAND_PAWN, EvalIndex::new_hand(Piece::B_PAWN));
    assert_eq!(EvalIndex::F_HAND_LANCE, EvalIndex::new_hand(Piece::B_LANCE));
    assert_eq!(EvalIndex::F_HAND_KNIGHT, EvalIndex::new_hand(Piece::B_KNIGHT));
    assert_eq!(EvalIndex::F_HAND_SILVER, EvalIndex::new_hand(Piece::B_SILVER));
    assert_eq!(EvalIndex::F_HAND_BISHOP, EvalIndex::new_hand(Piece::B_BISHOP));
    assert_eq!(EvalIndex::F_HAND_ROOK, EvalIndex::new_hand(Piece::B_ROOK));
    assert_eq!(EvalIndex::F_HAND_GOLD, EvalIndex::new_hand(Piece::B_GOLD));
    assert_eq!(EvalIndex::E_HAND_PAWN, EvalIndex::new_hand(Piece::W_PAWN));
    assert_eq!(EvalIndex::E_HAND_LANCE, EvalIndex::new_hand(Piece::W_LANCE));
    assert_eq!(EvalIndex::E_HAND_KNIGHT, EvalIndex::new_hand(Piece::W_KNIGHT));
    assert_eq!(EvalIndex::E_HAND_SILVER, EvalIndex::new_hand(Piece::W_SILVER));
    assert_eq!(EvalIndex::E_HAND_BISHOP, EvalIndex::new_hand(Piece::W_BISHOP));
    assert_eq!(EvalIndex::E_HAND_ROOK, EvalIndex::new_hand(Piece::W_ROOK));
    assert_eq!(EvalIndex::E_HAND_GOLD, EvalIndex::new_hand(Piece::W_GOLD));
}

#[test]
fn test_eval_index_inverse() {
    assert_eq!(
        EvalIndex(EvalIndex::F_HAND_PAWN.0 + 1).inverse(),
        EvalIndex(EvalIndex::E_HAND_PAWN.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_HAND_PAWN.0 + 1).inverse(),
        EvalIndex(EvalIndex::F_HAND_PAWN.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_HAND_LANCE.0 + 1).inverse(),
        EvalIndex(EvalIndex::E_HAND_LANCE.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_HAND_LANCE.0 + 1).inverse(),
        EvalIndex(EvalIndex::F_HAND_LANCE.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_HAND_KNIGHT.0 + 1).inverse(),
        EvalIndex(EvalIndex::E_HAND_KNIGHT.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_HAND_KNIGHT.0 + 1).inverse(),
        EvalIndex(EvalIndex::F_HAND_KNIGHT.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_HAND_SILVER.0 + 1).inverse(),
        EvalIndex(EvalIndex::E_HAND_SILVER.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_HAND_SILVER.0 + 1).inverse(),
        EvalIndex(EvalIndex::F_HAND_SILVER.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_HAND_GOLD.0 + 1).inverse(),
        EvalIndex(EvalIndex::E_HAND_GOLD.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_HAND_GOLD.0 + 1).inverse(),
        EvalIndex(EvalIndex::F_HAND_GOLD.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_HAND_BISHOP.0 + 1).inverse(),
        EvalIndex(EvalIndex::E_HAND_BISHOP.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_HAND_BISHOP.0 + 1).inverse(),
        EvalIndex(EvalIndex::F_HAND_BISHOP.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_HAND_ROOK.0 + 1).inverse(),
        EvalIndex(EvalIndex::E_HAND_ROOK.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_HAND_ROOK.0 + 1).inverse(),
        EvalIndex(EvalIndex::F_HAND_ROOK.0 + 1)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_PAWN.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::E_PAWN.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_PAWN.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::F_PAWN.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_LANCE.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::E_LANCE.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_LANCE.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::F_LANCE.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_KNIGHT.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::E_KNIGHT.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_KNIGHT.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::F_KNIGHT.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_SILVER.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::E_SILVER.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_SILVER.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::F_SILVER.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_GOLD.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::E_GOLD.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_GOLD.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::F_GOLD.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_BISHOP.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::E_BISHOP.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_BISHOP.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::F_BISHOP.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_ROOK.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::E_ROOK.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_ROOK.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::F_ROOK.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_HORSE.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::E_HORSE.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_HORSE.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::F_HORSE.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::F_DRAGON.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::E_DRAGON.0 + Square::SQ98.0 as usize)
    );
    assert_eq!(
        EvalIndex(EvalIndex::E_DRAGON.0 + Square::SQ12.0 as usize).inverse(),
        EvalIndex(EvalIndex::F_DRAGON.0 + Square::SQ98.0 as usize)
    );
}
