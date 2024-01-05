use crate::types::*;

pub const PAWN_VALUE: i32 = 100 * 9 / 10;
const LANCE_VALUE: i32 = 350 * 9 / 10;
const KNIGHT_VALUE: i32 = 450 * 9 / 10;
const SILVER_VALUE: i32 = 550 * 9 / 10;
const BISHOP_VALUE: i32 = 950 * 9 / 10;
const ROOK_VALUE: i32 = 1100 * 9 / 10;
const GOLD_VALUE: i32 = 600 * 9 / 10;
const PRO_PAWN_VALUE: i32 = 600 * 9 / 10;
const PRO_LANCE_VALUE: i32 = 600 * 9 / 10;
const PRO_KNIGHT_VALUE: i32 = 600 * 9 / 10;
const PRO_SILVER_VALUE: i32 = 600 * 9 / 10;
const HORSE_VALUE: i32 = 1050 * 9 / 10;
const DRAGON_VALUE: i32 = 1550 * 9 / 10;
const KING_VALUE: i32 = 0; // see_ge() needs that KING_VALUE == 0.

const PIECE_VALUES: [i32; Piece::NUM] = [
    0,
    PAWN_VALUE,
    LANCE_VALUE,
    KNIGHT_VALUE,
    SILVER_VALUE,
    BISHOP_VALUE,
    ROOK_VALUE,
    GOLD_VALUE,
    KING_VALUE,
    PRO_PAWN_VALUE,
    PRO_LANCE_VALUE,
    PRO_KNIGHT_VALUE,
    PRO_SILVER_VALUE,
    HORSE_VALUE,
    DRAGON_VALUE,
    0,
    0,
    PAWN_VALUE,
    LANCE_VALUE,
    KNIGHT_VALUE,
    SILVER_VALUE,
    BISHOP_VALUE,
    ROOK_VALUE,
    GOLD_VALUE,
    KING_VALUE,
    PRO_PAWN_VALUE,
    PRO_LANCE_VALUE,
    PRO_KNIGHT_VALUE,
    PRO_SILVER_VALUE,
    HORSE_VALUE,
    DRAGON_VALUE,
];

#[allow(dead_code)]
pub fn piece_value(pc: Piece) -> Value {
    debug_assert!(0 <= pc.0);
    debug_assert!((pc.0 as usize) < Piece::NUM);
    unsafe { Value(*PIECE_VALUES.get_unchecked(pc.0 as usize)) }
}
pub fn piece_type_value(pt: PieceType) -> Value {
    debug_assert!(0 <= pt.0);
    debug_assert!((pt.0 as usize) < PieceType::NUM);
    unsafe { Value(*PIECE_VALUES.get_unchecked(pt.0 as usize)) }
}

const CAPTURE_PAWN_VALUE: i32 = PAWN_VALUE * 2;
const CAPTURE_LANCE_VALUE: i32 = LANCE_VALUE * 2;
const CAPTURE_KNIGHT_VALUE: i32 = KNIGHT_VALUE * 2;
const CAPTURE_SILVER_VALUE: i32 = SILVER_VALUE * 2;
const CAPTURE_BISHOP_VALUE: i32 = BISHOP_VALUE * 2;
const CAPTURE_ROOK_VALUE: i32 = ROOK_VALUE * 2;
const CAPTURE_GOLD_VALUE: i32 = GOLD_VALUE * 2;
const CAPTURE_PRO_PAWN_VALUE: i32 = PRO_PAWN_VALUE + PAWN_VALUE;
const CAPTURE_PRO_LANCE_VALUE: i32 = PRO_LANCE_VALUE + LANCE_VALUE;
const CAPTURE_PRO_KNIGHT_VALUE: i32 = PRO_KNIGHT_VALUE + KNIGHT_VALUE;
const CAPTURE_PRO_SILVER_VALUE: i32 = PRO_SILVER_VALUE + SILVER_VALUE;
const CAPTURE_HORSE_VALUE: i32 = HORSE_VALUE + BISHOP_VALUE;
const CAPTURE_DRAGON_VALUE: i32 = DRAGON_VALUE + ROOK_VALUE;
const CAPTURE_KING_VALUE: i32 = KING_VALUE * 2;

const CAPTURE_PIECE_VALUES: [i32; Piece::NUM] = [
    0,
    CAPTURE_PAWN_VALUE,
    CAPTURE_LANCE_VALUE,
    CAPTURE_KNIGHT_VALUE,
    CAPTURE_SILVER_VALUE,
    CAPTURE_BISHOP_VALUE,
    CAPTURE_ROOK_VALUE,
    CAPTURE_GOLD_VALUE,
    CAPTURE_KING_VALUE,
    CAPTURE_PRO_PAWN_VALUE,
    CAPTURE_PRO_LANCE_VALUE,
    CAPTURE_PRO_KNIGHT_VALUE,
    CAPTURE_PRO_SILVER_VALUE,
    CAPTURE_HORSE_VALUE,
    CAPTURE_DRAGON_VALUE,
    0,
    0,
    CAPTURE_PAWN_VALUE,
    CAPTURE_LANCE_VALUE,
    CAPTURE_KNIGHT_VALUE,
    CAPTURE_SILVER_VALUE,
    CAPTURE_BISHOP_VALUE,
    CAPTURE_ROOK_VALUE,
    CAPTURE_GOLD_VALUE,
    CAPTURE_KING_VALUE,
    CAPTURE_PRO_PAWN_VALUE,
    CAPTURE_PRO_LANCE_VALUE,
    CAPTURE_PRO_KNIGHT_VALUE,
    CAPTURE_PRO_SILVER_VALUE,
    CAPTURE_HORSE_VALUE,
    CAPTURE_DRAGON_VALUE,
];

pub fn capture_piece_value(pc: Piece) -> Value {
    debug_assert!(0 <= pc.0);
    debug_assert!((pc.0 as usize) < Piece::NUM);
    unsafe { Value(*CAPTURE_PIECE_VALUES.get_unchecked(pc.0 as usize)) }
}
pub fn capture_piece_type_value(pt: PieceType) -> Value {
    debug_assert!(0 <= pt.0);
    debug_assert!((pt.0 as usize) < PieceType::NUM);
    unsafe { Value(*CAPTURE_PIECE_VALUES.get_unchecked(pt.0 as usize)) }
}

const PROMOTE_PAWN_VALUE: i32 = PRO_PAWN_VALUE - PAWN_VALUE;
const PROMOTE_LANCE_VALUE: i32 = PRO_LANCE_VALUE - LANCE_VALUE;
const PROMOTE_KNIGHT_VALUE: i32 = PRO_KNIGHT_VALUE - KNIGHT_VALUE;
const PROMOTE_SILVER_VALUE: i32 = PRO_SILVER_VALUE - SILVER_VALUE;
const PROMOTE_BISHOP_VALUE: i32 = HORSE_VALUE - BISHOP_VALUE;
const PROMOTE_ROOK_VALUE: i32 = DRAGON_VALUE - ROOK_VALUE;

const PROMOTE_PIECE_VALUES: [i32; 7] = [
    0,
    PROMOTE_PAWN_VALUE,
    PROMOTE_LANCE_VALUE,
    PROMOTE_KNIGHT_VALUE,
    PROMOTE_SILVER_VALUE,
    PROMOTE_BISHOP_VALUE,
    PROMOTE_ROOK_VALUE,
];

pub fn promote_piece_type_value(pt: PieceType) -> Value {
    debug_assert!(0 <= pt.0);
    debug_assert!((pt.0 as usize) < PROMOTE_PIECE_VALUES.len());
    unsafe { Value(*PROMOTE_PIECE_VALUES.get_unchecked(pt.0 as usize)) }
}

pub fn lva_value(pt: PieceType) -> Value {
    match pt {
        PieceType::PAWN => Value(1),
        PieceType::LANCE => Value(2),
        PieceType::KNIGHT => Value(3),
        PieceType::SILVER => Value(4),
        PieceType::PRO_SILVER => Value(5),
        PieceType::PRO_KNIGHT => Value(5),
        PieceType::PRO_LANCE => Value(5),
        PieceType::PRO_PAWN => Value(5),
        PieceType::GOLD => Value(6),
        PieceType::BISHOP => Value(7),
        PieceType::HORSE => Value(8),
        PieceType::ROOK => Value(9),
        PieceType::DRAGON => Value(10),
        PieceType::KING => Value(10000),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_value() {
        assert_eq!(piece_type_value(PieceType::PAWN), Value(PAWN_VALUE));
        assert_eq!(piece_type_value(PieceType::LANCE), Value(LANCE_VALUE));
        assert_eq!(piece_type_value(PieceType::KNIGHT), Value(KNIGHT_VALUE));
        assert_eq!(piece_type_value(PieceType::SILVER), Value(SILVER_VALUE));
        assert_eq!(piece_type_value(PieceType::BISHOP), Value(BISHOP_VALUE));
        assert_eq!(piece_type_value(PieceType::ROOK), Value(ROOK_VALUE));
        assert_eq!(piece_type_value(PieceType::GOLD), Value(GOLD_VALUE));
        assert_eq!(piece_type_value(PieceType::KING), Value(KING_VALUE));
        assert_eq!(piece_type_value(PieceType::PRO_PAWN), Value(PRO_PAWN_VALUE));
        assert_eq!(piece_type_value(PieceType::PRO_LANCE), Value(PRO_LANCE_VALUE));
        assert_eq!(piece_type_value(PieceType::PRO_KNIGHT), Value(PRO_KNIGHT_VALUE));
        assert_eq!(piece_type_value(PieceType::PRO_SILVER), Value(PRO_SILVER_VALUE));
        assert_eq!(piece_type_value(PieceType::HORSE), Value(HORSE_VALUE));
        assert_eq!(piece_type_value(PieceType::DRAGON), Value(DRAGON_VALUE));

        assert_eq!(piece_value(Piece::EMPTY), Value(0));
        assert_eq!(piece_value(Piece::B_PAWN), Value(PAWN_VALUE));
        assert_eq!(piece_value(Piece::B_LANCE), Value(LANCE_VALUE));
        assert_eq!(piece_value(Piece::B_KNIGHT), Value(KNIGHT_VALUE));
        assert_eq!(piece_value(Piece::B_SILVER), Value(SILVER_VALUE));
        assert_eq!(piece_value(Piece::B_BISHOP), Value(BISHOP_VALUE));
        assert_eq!(piece_value(Piece::B_ROOK), Value(ROOK_VALUE));
        assert_eq!(piece_value(Piece::B_GOLD), Value(GOLD_VALUE));
        assert_eq!(piece_value(Piece::B_KING), Value(KING_VALUE));
        assert_eq!(piece_value(Piece::B_PRO_PAWN), Value(PRO_PAWN_VALUE));
        assert_eq!(piece_value(Piece::B_PRO_LANCE), Value(PRO_LANCE_VALUE));
        assert_eq!(piece_value(Piece::B_PRO_KNIGHT), Value(PRO_KNIGHT_VALUE));
        assert_eq!(piece_value(Piece::B_PRO_SILVER), Value(PRO_SILVER_VALUE));
        assert_eq!(piece_value(Piece::B_HORSE), Value(HORSE_VALUE));
        assert_eq!(piece_value(Piece::B_DRAGON), Value(DRAGON_VALUE));

        assert_eq!(piece_value(Piece::W_PAWN), Value(PAWN_VALUE));
        assert_eq!(piece_value(Piece::W_LANCE), Value(LANCE_VALUE));
        assert_eq!(piece_value(Piece::W_KNIGHT), Value(KNIGHT_VALUE));
        assert_eq!(piece_value(Piece::W_SILVER), Value(SILVER_VALUE));
        assert_eq!(piece_value(Piece::W_BISHOP), Value(BISHOP_VALUE));
        assert_eq!(piece_value(Piece::W_ROOK), Value(ROOK_VALUE));
        assert_eq!(piece_value(Piece::W_GOLD), Value(GOLD_VALUE));
        assert_eq!(piece_value(Piece::W_KING), Value(KING_VALUE));
        assert_eq!(piece_value(Piece::W_PRO_PAWN), Value(PRO_PAWN_VALUE));
        assert_eq!(piece_value(Piece::W_PRO_LANCE), Value(PRO_LANCE_VALUE));
        assert_eq!(piece_value(Piece::W_PRO_KNIGHT), Value(PRO_KNIGHT_VALUE));
        assert_eq!(piece_value(Piece::W_PRO_SILVER), Value(PRO_SILVER_VALUE));
        assert_eq!(piece_value(Piece::W_HORSE), Value(HORSE_VALUE));
        assert_eq!(piece_value(Piece::W_DRAGON), Value(DRAGON_VALUE));
    }

    #[test]
    fn test_capture_piece_value() {
        assert_eq!(capture_piece_type_value(PieceType::PAWN), Value(CAPTURE_PAWN_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::LANCE), Value(CAPTURE_LANCE_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::KNIGHT), Value(CAPTURE_KNIGHT_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::SILVER), Value(CAPTURE_SILVER_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::BISHOP), Value(CAPTURE_BISHOP_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::ROOK), Value(CAPTURE_ROOK_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::GOLD), Value(CAPTURE_GOLD_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::KING), Value(CAPTURE_KING_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::PRO_PAWN), Value(CAPTURE_PRO_PAWN_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::PRO_LANCE), Value(CAPTURE_PRO_LANCE_VALUE));
        assert_eq!(
            capture_piece_type_value(PieceType::PRO_KNIGHT),
            Value(CAPTURE_PRO_KNIGHT_VALUE)
        );
        assert_eq!(
            capture_piece_type_value(PieceType::PRO_SILVER),
            Value(CAPTURE_PRO_SILVER_VALUE)
        );
        assert_eq!(capture_piece_type_value(PieceType::HORSE), Value(CAPTURE_HORSE_VALUE));
        assert_eq!(capture_piece_type_value(PieceType::DRAGON), Value(CAPTURE_DRAGON_VALUE));

        assert_eq!(capture_piece_value(Piece::EMPTY), Value(0));
        assert_eq!(capture_piece_value(Piece::B_PAWN), Value(CAPTURE_PAWN_VALUE));
        assert_eq!(capture_piece_value(Piece::B_LANCE), Value(CAPTURE_LANCE_VALUE));
        assert_eq!(capture_piece_value(Piece::B_KNIGHT), Value(CAPTURE_KNIGHT_VALUE));
        assert_eq!(capture_piece_value(Piece::B_SILVER), Value(CAPTURE_SILVER_VALUE));
        assert_eq!(capture_piece_value(Piece::B_BISHOP), Value(CAPTURE_BISHOP_VALUE));
        assert_eq!(capture_piece_value(Piece::B_ROOK), Value(CAPTURE_ROOK_VALUE));
        assert_eq!(capture_piece_value(Piece::B_GOLD), Value(CAPTURE_GOLD_VALUE));
        assert_eq!(capture_piece_value(Piece::B_KING), Value(CAPTURE_KING_VALUE));
        assert_eq!(capture_piece_value(Piece::B_PRO_PAWN), Value(CAPTURE_PRO_PAWN_VALUE));
        assert_eq!(capture_piece_value(Piece::B_PRO_LANCE), Value(CAPTURE_PRO_LANCE_VALUE));
        assert_eq!(capture_piece_value(Piece::B_PRO_KNIGHT), Value(CAPTURE_PRO_KNIGHT_VALUE));
        assert_eq!(capture_piece_value(Piece::B_PRO_SILVER), Value(CAPTURE_PRO_SILVER_VALUE));
        assert_eq!(capture_piece_value(Piece::B_HORSE), Value(CAPTURE_HORSE_VALUE));
        assert_eq!(capture_piece_value(Piece::B_DRAGON), Value(CAPTURE_DRAGON_VALUE));

        assert_eq!(capture_piece_value(Piece::W_PAWN), Value(CAPTURE_PAWN_VALUE));
        assert_eq!(capture_piece_value(Piece::W_LANCE), Value(CAPTURE_LANCE_VALUE));
        assert_eq!(capture_piece_value(Piece::W_KNIGHT), Value(CAPTURE_KNIGHT_VALUE));
        assert_eq!(capture_piece_value(Piece::W_SILVER), Value(CAPTURE_SILVER_VALUE));
        assert_eq!(capture_piece_value(Piece::W_BISHOP), Value(CAPTURE_BISHOP_VALUE));
        assert_eq!(capture_piece_value(Piece::W_ROOK), Value(CAPTURE_ROOK_VALUE));
        assert_eq!(capture_piece_value(Piece::W_GOLD), Value(CAPTURE_GOLD_VALUE));
        assert_eq!(capture_piece_value(Piece::W_KING), Value(CAPTURE_KING_VALUE));
        assert_eq!(capture_piece_value(Piece::W_PRO_PAWN), Value(CAPTURE_PRO_PAWN_VALUE));
        assert_eq!(capture_piece_value(Piece::W_PRO_LANCE), Value(CAPTURE_PRO_LANCE_VALUE));
        assert_eq!(capture_piece_value(Piece::W_PRO_KNIGHT), Value(CAPTURE_PRO_KNIGHT_VALUE));
        assert_eq!(capture_piece_value(Piece::W_PRO_SILVER), Value(CAPTURE_PRO_SILVER_VALUE));
        assert_eq!(capture_piece_value(Piece::W_HORSE), Value(CAPTURE_HORSE_VALUE));
        assert_eq!(capture_piece_value(Piece::W_DRAGON), Value(CAPTURE_DRAGON_VALUE));
    }

    #[test]
    fn test_promote_piece_value() {
        assert_eq!(promote_piece_type_value(PieceType::PAWN), Value(PROMOTE_PAWN_VALUE));
        assert_eq!(promote_piece_type_value(PieceType::LANCE), Value(PROMOTE_LANCE_VALUE));
        assert_eq!(promote_piece_type_value(PieceType::KNIGHT), Value(PROMOTE_KNIGHT_VALUE));
        assert_eq!(promote_piece_type_value(PieceType::SILVER), Value(PROMOTE_SILVER_VALUE));
        assert_eq!(promote_piece_type_value(PieceType::BISHOP), Value(PROMOTE_BISHOP_VALUE));
        assert_eq!(promote_piece_type_value(PieceType::ROOK), Value(PROMOTE_ROOK_VALUE));
    }
}
