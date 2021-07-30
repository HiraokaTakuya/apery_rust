use crate::position::*;
use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct UsiMove(String);

impl UsiMove {
    fn as_str(&self) -> &str {
        &self.0
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CsaMove(String);

// xxxxxxxx xxxxxxxx xxxxxxxx x1111111  to
// xxxxxxxx xxxxxxxx xxxxxxxx 1xxxxxxx  promote flag
// xxxxxxxx xxxxxxxx xxxxxxx1 xxxxxxxx  drop flag
// xxxxxxxx xxxxxxxx 1111111x xxxxxxxx  from or piece_dropped
// xxxxxxxx xxx11111 xxxxxxxx xxxxxxxx  moved piece (If this move is promotion. moved piece is unpromoted piece. If drop, it's 0.)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move(pub std::num::NonZeroU32);

impl Move {
    const TO_MASK: u32 = 0x0000_007f;
    const FROM_MASK: u32 = 0x0000_fe00;
    const PIECE_TYPE_DROPPED_MASK: u32 = 0x0000_1e00;
    const PIECE_DROPPED_MASK: u32 = 0x0000_3e00;
    const MOVED_PIECE_MASK: u32 = 0x001f_0000;
    const PROMOTE_FLAG: u32 = 1 << 7;
    const DROP_FLAG: u32 = 1 << 8;
    const FROM_SHIFT: i32 = 9;
    const PIECE_TYPE_DROPPED_SHIFT: i32 = 9;
    const PIECE_DROPPED_SHIFT: i32 = 9;
    pub const MOVED_PIECE_SHIFT: i32 = 16;

    pub const NULL: Move = Move(unsafe { std::num::NonZeroU32::new_unchecked(1 | (1 << Move::FROM_SHIFT)) }); // !is_promotion() && to() == from()
    pub const WIN: Move = Move(unsafe { std::num::NonZeroU32::new_unchecked(2 | (2 << Move::FROM_SHIFT)) });
    pub const RESIGN: Move = Move(unsafe { std::num::NonZeroU32::new_unchecked(3 | (3 << Move::FROM_SHIFT)) });

    pub fn new_unpromote(from: Square, to: Square, pc: Piece) -> Move {
        Move(unsafe {
            std::num::NonZeroU32::new_unchecked(
                ((pc.0 as u32) << Move::MOVED_PIECE_SHIFT) | ((from.0 as u32) << Move::FROM_SHIFT) | (to.0 as u32),
            )
        })
    }
    #[inline]
    pub fn new_promote(from: Square, to: Square, pc: Piece) -> Move {
        Move(unsafe { std::num::NonZeroU32::new_unchecked(Move::PROMOTE_FLAG | Move::new_unpromote(from, to, pc).0.get()) })
    }
    pub fn new_drop(pc: Piece, to: Square) -> Move {
        Move(unsafe {
            std::num::NonZeroU32::new_unchecked(Move::DROP_FLAG | ((pc.0 as u32) << Move::PIECE_DROPPED_SHIFT) | (to.0 as u32))
        })
    }
    pub fn new_from_usi_str(s: &str, pos: &Position) -> Option<Move> {
        let m;
        let v: Vec<char> = s.chars().collect();
        if v.len() < 4 {
            // Any move is illegal.
            return None;
        }
        if let Some(pt) = PieceType::new_from_str_for_drop_move(&v[0].to_string()) {
            let pc = Piece::new(pos.side_to_move(), pt);
            // Drop move.
            if v[1] != '*' {
                return None;
            }
            if v.len() != 4 {
                return None;
            }
            let file = File::new_from_usi_char(v[2])?;
            let rank = Rank::new_from_usi_char(v[3])?;
            let to = Square::new(file, rank);
            m = Move::new_drop(pc, to);
        } else {
            // Not drop move.
            let file_from = File::new_from_usi_char(v[0])?;
            let rank_from = Rank::new_from_usi_char(v[1])?;
            let file_to = File::new_from_usi_char(v[2])?;
            let rank_to = Rank::new_from_usi_char(v[3])?;
            let from = Square::new(file_from, rank_from);
            let to = Square::new(file_to, rank_to);
            let pc = pos.piece_on(from);
            if v.len() == 4 {
                // Unpromote move.
                m = Move::new_unpromote(from, to, pc);
            } else if v.len() == 5 {
                if v[4] != '+' {
                    return None;
                }
                m = Move::new_promote(from, to, pc);
            } else {
                return None;
            }
        }
        if !pos.pseudo_legal::<NotSearchingType>(m) || !pos.legal(m) {
            return None;
        }
        Some(m)
    }
    pub fn new_from_usi(usi_move: &UsiMove, pos: &Position) -> Option<Move> {
        Self::new_from_usi_str(usi_move.as_str(), pos)
    }
    pub fn new_from_csa_str(s: &str, pos: &Position) -> Option<Move> {
        let m;
        let mut v: Vec<char> = s.chars().collect();
        match v.len() {
            len if len < 6 => {
                // Any move is illegal.
                return None;
            }
            len if len > 6 => {
                v.truncate(6);
            }
            _ => {}
        }
        let v = v;
        let pc = {
            let pt = PieceType::new_from_csa_str(&v[4..6].iter().collect::<String>())?;
            Piece::new(pos.side_to_move(), pt)
        };
        let to = {
            let file_to = File::new_from_csa_char(v[2])?;
            let rank_to = Rank::new_from_csa_char(v[3])?;
            Square::new(file_to, rank_to)
        };
        if v[0] == '0' && v[1] == '0' {
            m = Move::new_drop(pc, to);
        } else {
            let from = {
                let file_from = File::new_from_csa_char(v[0])?;
                let rank_from = Rank::new_from_csa_char(v[1])?;
                Square::new(file_from, rank_from)
            };
            let is_promote = {
                let pc_from = pos.piece_on(from);
                if pc_from == pc {
                    false
                } else if pc_from.is_promotable() && pc_from.to_promote() == pc {
                    true
                } else {
                    return None;
                }
            };
            if is_promote {
                m = Move::new_promote(from, to, pc);
            } else {
                m = Move::new_unpromote(from, to, pc);
            }
        }

        if !pos.pseudo_legal::<NotSearchingType>(m) || !pos.legal(m) {
            return None;
        }

        Some(m)
    }
    pub fn reverse(self) -> Move {
        let pc = Piece(((self.0.get() & Move::MOVED_PIECE_MASK) >> Move::MOVED_PIECE_SHIFT) as i32);
        Move::new_unpromote(self.to(), self.to(), pc)
    }
    #[inline]
    pub fn to(self) -> Square {
        Square((self.0.get() & Move::TO_MASK) as i32)
    }
    pub fn from(self) -> Square {
        Square(((self.0.get() & Move::FROM_MASK) >> Move::FROM_SHIFT) as i32)
    }
    pub fn piece_dropped(self) -> Piece {
        Piece(((self.0.get() & Move::PIECE_DROPPED_MASK) >> Move::PIECE_DROPPED_SHIFT) as i32)
    }
    pub fn piece_type_dropped(self) -> PieceType {
        PieceType(((self.0.get() & Move::PIECE_TYPE_DROPPED_MASK) >> Move::PIECE_TYPE_DROPPED_SHIFT) as i32)
    }
    pub fn piece_moved_before_move(self) -> Piece {
        if self.is_drop() {
            self.piece_dropped()
        } else {
            Piece(((self.0.get() & Move::MOVED_PIECE_MASK) >> Move::MOVED_PIECE_SHIFT) as i32)
        }
    }
    pub fn piece_moved_after_move(self) -> Piece {
        if self.is_drop() {
            self.piece_dropped()
        } else {
            const SHIFT: i32 = 4;
            debug_assert_eq!(Move::PROMOTE_FLAG >> SHIFT, Piece::PROMOTION as u32);
            Piece(
                (((self.0.get() & Move::MOVED_PIECE_MASK) >> Move::MOVED_PIECE_SHIFT)
                    | ((self.0.get() & Move::PROMOTE_FLAG) >> SHIFT)) as i32,
            )
        }
    }
    pub fn is_drop(self) -> bool {
        (self.0.get() & Move::DROP_FLAG) != 0
    }
    pub fn is_promotion(self) -> bool {
        (self.0.get() & Move::PROMOTE_FLAG) != 0
    }
    // You can use this function only before Position::do_move() with this move.
    pub fn is_capture(self, pos: &Position) -> bool {
        pos.piece_on(self.to()) != Piece::EMPTY
    }
    pub fn is_pawn_promotion(self) -> bool {
        self.is_promotion() && PieceType::new(self.piece_moved_before_move()) == PieceType::PAWN
    }
    // You can use this function only before Position::do_move() with this move.
    pub fn is_capture_or_pawn_promotion(self, pos: &Position) -> bool {
        self.is_capture(pos) || self.is_pawn_promotion()
    }
    pub fn to_usi(self) -> UsiMove {
        let mut s = "".to_string();
        if self.is_drop() {
            let pt = self.piece_type_dropped();
            s += pt.to_usi_str();
            s += "*";
            s += &self.to().to_usi_string();
        } else {
            s += &self.from().to_usi_string();
            s += &self.to().to_usi_string();
            if self.is_promotion() {
                s += "+";
            }
        }
        UsiMove(s)
    }
    pub fn to_usi_string(self) -> String {
        self.to_usi().0
    }
    #[allow(dead_code)]
    pub fn to_csa_string(self, pos: &Position) -> String {
        let mut s = "".to_string();
        let pt;
        if self.is_drop() {
            s += "00";
            pt = self.piece_type_dropped();
        } else {
            s += &self.from().to_csa_string();
            let pt_tmp = PieceType::new(pos.piece_on(self.from()));
            if self.is_promotion() {
                pt = pt_tmp.to_promote();
            } else {
                pt = pt_tmp;
            }
        }
        s += &self.to().to_csa_string();
        s += pt.to_csa_str();
        s
    }
}

pub trait NonZeroUnwrapUnchecked {
    fn non_zero_unwrap_unchecked(self) -> Move;
}

impl NonZeroUnwrapUnchecked for Option<Move> {
    #[inline]
    fn non_zero_unwrap_unchecked(self) -> Move {
        unsafe { std::mem::transmute::<Option<Move>, Move>(self) }
    }
}

pub trait IsNormalMove {
    fn is_normal_move(&self) -> bool;
}

impl IsNormalMove for Option<Move> {
    fn is_normal_move(&self) -> bool {
        let val = self.non_zero_unwrap_unchecked().0.get();
        let ret = (val & 0x1ff) != (val >> 9);
        debug_assert_eq!(
            ret,
            self.is_some() && self.unwrap() != Move::NULL && self.unwrap() != Move::WIN && self.unwrap() != Move::RESIGN
        );
        ret
    }
}

pub struct ExtMove {
    pub mv: Move,
    pub score: i32,
}

impl ExtMove {
    pub const MAX_LEGAL_MOVES: usize = 593 + 1;
}

impl Ord for ExtMove {
    fn cmp(&self, other: &ExtMove) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for ExtMove {
    fn partial_cmp(&self, other: &ExtMove) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExtMove {
    fn eq(&self, other: &ExtMove) -> bool {
        self.score == other.score
    }
}

impl Eq for ExtMove {}

impl Clone for ExtMove {
    fn clone(&self) -> ExtMove {
        ExtMove {
            mv: self.mv,
            score: self.score,
        }
    }
}
