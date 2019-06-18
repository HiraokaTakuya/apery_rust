use crate::types::*;

// xxxxxxxx xxxxxxxx xxxxxxxx xxx11111  Pawn
// xxxxxxxx xxxxxxxx xxxxxxx1 11xxxxxx  Lance
// xxxxxxxx xxxxxxxx xxx111xx xxxxxxxx  Knight
// xxxxxxxx xxxxxxx1 11xxxxxx xxxxxxxx  Silver
// xxxxxxxx xxxx11xx xxxxxxxx xxxxxxxx  Bishop
// xxxxxxxx x11xxxxx xxxxxxxx xxxxxxxx  Rook
// xxxxx111 xxxxxxxx xxxxxxxx xxxxxxxx  Gold
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hand(pub u32);

impl Hand {
    const PAWN_REQUIRE_BITS: u32 = 5;
    const LANCE_REQUIRE_BITS: u32 = 3;
    const KNIGHT_REQUIRE_BITS: u32 = 3;
    const SILVER_REQUIRE_BITS: u32 = 3;
    const BISHOP_REQUIRE_BITS: u32 = 2;
    const ROOK_REQUIRE_BITS: u32 = 2;
    const GOLD_REQUIRE_BITS: u32 = 3;

    const PAWN_SHIFT_BITS: u32 = 0;
    const LANCE_SHIFT_BITS: u32 = Hand::PAWN_SHIFT_BITS + Hand::PAWN_REQUIRE_BITS + 1;
    const KNIGHT_SHIFT_BITS: u32 = Hand::LANCE_SHIFT_BITS + Hand::LANCE_REQUIRE_BITS + 1;
    const SILVER_SHIFT_BITS: u32 = Hand::KNIGHT_SHIFT_BITS + Hand::KNIGHT_REQUIRE_BITS + 1;
    const BISHOP_SHIFT_BITS: u32 = Hand::SILVER_SHIFT_BITS + Hand::SILVER_REQUIRE_BITS + 1;
    const ROOK_SHIFT_BITS: u32 = Hand::BISHOP_SHIFT_BITS + Hand::BISHOP_REQUIRE_BITS + 1;
    const GOLD_SHIFT_BITS: u32 = Hand::ROOK_SHIFT_BITS + Hand::ROOK_REQUIRE_BITS + 1;

    const PAWN_MASK: u32 = ((1 << Hand::PAWN_REQUIRE_BITS) - 1) << Hand::PAWN_SHIFT_BITS;
    const LANCE_MASK: u32 = ((1 << Hand::LANCE_REQUIRE_BITS) - 1) << Hand::LANCE_SHIFT_BITS;
    const KNIGHT_MASK: u32 = ((1 << Hand::KNIGHT_REQUIRE_BITS) - 1) << Hand::KNIGHT_SHIFT_BITS;
    const SILVER_MASK: u32 = ((1 << Hand::SILVER_REQUIRE_BITS) - 1) << Hand::SILVER_SHIFT_BITS;
    const BISHOP_MASK: u32 = ((1 << Hand::BISHOP_REQUIRE_BITS) - 1) << Hand::BISHOP_SHIFT_BITS;
    const ROOK_MASK: u32 = ((1 << Hand::ROOK_REQUIRE_BITS) - 1) << Hand::ROOK_SHIFT_BITS;
    const GOLD_MASK: u32 = ((1 << Hand::GOLD_REQUIRE_BITS) - 1) << Hand::GOLD_SHIFT_BITS;

    const EXCEPT_PAWN_MASK: u32 = (Hand::LANCE_MASK
        | Hand::KNIGHT_MASK
        | Hand::SILVER_MASK
        | Hand::BISHOP_MASK
        | Hand::ROOK_MASK
        | Hand::GOLD_MASK);
    const BORROW_MASK: u32 = ((Hand::PAWN_MASK + (1 << Hand::PAWN_SHIFT_BITS))
        | (Hand::LANCE_MASK + (1 << Hand::LANCE_SHIFT_BITS))
        | (Hand::KNIGHT_MASK + (1 << Hand::KNIGHT_SHIFT_BITS))
        | (Hand::SILVER_MASK + (1 << Hand::SILVER_SHIFT_BITS))
        | (Hand::BISHOP_MASK + (1 << Hand::BISHOP_SHIFT_BITS))
        | (Hand::ROOK_MASK + (1 << Hand::ROOK_SHIFT_BITS))
        | (Hand::GOLD_MASK + (1 << Hand::GOLD_SHIFT_BITS)));

    const PAWN_ONE: u32 = 1 << Hand::PAWN_SHIFT_BITS;
    const LANCE_ONE: u32 = 1 << Hand::LANCE_SHIFT_BITS;
    const KNIGHT_ONE: u32 = 1 << Hand::KNIGHT_SHIFT_BITS;
    const SILVER_ONE: u32 = 1 << Hand::SILVER_SHIFT_BITS;
    const BISHOP_ONE: u32 = 1 << Hand::BISHOP_SHIFT_BITS;
    const ROOK_ONE: u32 = 1 << Hand::ROOK_SHIFT_BITS;
    const GOLD_ONE: u32 = 1 << Hand::GOLD_SHIFT_BITS;

    pub fn num(self, pt: PieceType) -> u32 {
        let (mask, shift) = match pt {
            PieceType::PAWN => (Hand::PAWN_MASK, Hand::PAWN_SHIFT_BITS),
            PieceType::LANCE => (Hand::LANCE_MASK, Hand::LANCE_SHIFT_BITS),
            PieceType::KNIGHT => (Hand::KNIGHT_MASK, Hand::KNIGHT_SHIFT_BITS),
            PieceType::SILVER => (Hand::SILVER_MASK, Hand::SILVER_SHIFT_BITS),
            PieceType::BISHOP => (Hand::BISHOP_MASK, Hand::BISHOP_SHIFT_BITS),
            PieceType::ROOK => (Hand::ROOK_MASK, Hand::ROOK_SHIFT_BITS),
            PieceType::GOLD => (Hand::GOLD_MASK, Hand::GOLD_SHIFT_BITS),
            _ => unreachable!(),
        };
        (self.0 & mask) >> shift
    }

    pub fn exist(self, pt: PieceType) -> bool {
        let mask = match pt {
            PieceType::PAWN => Hand::PAWN_MASK,
            PieceType::LANCE => Hand::LANCE_MASK,
            PieceType::KNIGHT => Hand::KNIGHT_MASK,
            PieceType::SILVER => Hand::SILVER_MASK,
            PieceType::BISHOP => Hand::BISHOP_MASK,
            PieceType::ROOK => Hand::ROOK_MASK,
            PieceType::GOLD => Hand::GOLD_MASK,
            _ => unreachable!(),
        };
        (self.0 & mask) != 0
    }

    pub fn except_pawn_exist(self) -> bool {
        (self.0 & Hand::EXCEPT_PAWN_MASK) != 0
    }

    pub fn set(&mut self, pt: PieceType, num: u32) {
        self.0 |= num
            << match pt {
                PieceType::PAWN => Hand::PAWN_SHIFT_BITS,
                PieceType::LANCE => Hand::LANCE_SHIFT_BITS,
                PieceType::KNIGHT => Hand::KNIGHT_SHIFT_BITS,
                PieceType::SILVER => Hand::SILVER_SHIFT_BITS,
                PieceType::BISHOP => Hand::BISHOP_SHIFT_BITS,
                PieceType::ROOK => Hand::ROOK_SHIFT_BITS,
                PieceType::GOLD => Hand::GOLD_SHIFT_BITS,
                _ => unreachable!(),
            };
    }

    pub fn plus_one(&mut self, pt: PieceType) {
        self.0 += match pt {
            PieceType::PAWN => Hand::PAWN_ONE,
            PieceType::LANCE => Hand::LANCE_ONE,
            PieceType::KNIGHT => Hand::KNIGHT_ONE,
            PieceType::SILVER => Hand::SILVER_ONE,
            PieceType::BISHOP | PieceType::HORSE => Hand::BISHOP_ONE,
            PieceType::ROOK | PieceType::DRAGON => Hand::ROOK_ONE,
            PieceType::GOLD
            | PieceType::PRO_PAWN
            | PieceType::PRO_LANCE
            | PieceType::PRO_KNIGHT
            | PieceType::PRO_SILVER => Hand::GOLD_ONE,
            _ => unreachable!(),
        };
    }

    pub fn minus_one(&mut self, pt: PieceType) {
        self.0 -= match pt {
            PieceType::PAWN => Hand::PAWN_ONE,
            PieceType::LANCE => Hand::LANCE_ONE,
            PieceType::KNIGHT => Hand::KNIGHT_ONE,
            PieceType::SILVER => Hand::SILVER_ONE,
            PieceType::BISHOP | PieceType::HORSE => Hand::BISHOP_ONE,
            PieceType::ROOK | PieceType::DRAGON => Hand::ROOK_ONE,
            PieceType::GOLD
            | PieceType::PRO_PAWN
            | PieceType::PRO_LANCE
            | PieceType::PRO_KNIGHT
            | PieceType::PRO_SILVER => Hand::GOLD_ONE,
            _ => unreachable!(),
        };
    }

    pub fn is_equal_or_superior(self, other: Hand) -> bool {
        (self.0.wrapping_sub(other.0) & Hand::BORROW_MASK) == 0
    }
}

#[test]
fn test_hand_shift_bits() {
    assert_eq!(Hand::PAWN_SHIFT_BITS, 0);
    assert_eq!(Hand::BISHOP_SHIFT_BITS, 18);
    assert_eq!(Hand::ROOK_SHIFT_BITS, 21);
}

#[test]
fn test_hand_num() {
    let hand = Hand(3 << Hand::LANCE_SHIFT_BITS);
    assert_eq!(hand.num(PieceType::PAWN), 0);
    assert_eq!(hand.num(PieceType::LANCE), 3);
    assert_eq!(hand.num(PieceType::KNIGHT), 0);
}

#[test]
fn test_hand_set() {
    let mut hand = Hand(0);
    hand.set(PieceType::LANCE, 2);
    hand.set(PieceType::GOLD, 4);
    hand.set(PieceType::BISHOP, 1);
    hand.minus_one(PieceType::GOLD);
    hand.plus_one(PieceType::BISHOP);
    assert_eq!(hand.num(PieceType::LANCE), 2);
    assert_eq!(hand.num(PieceType::GOLD), 3);
    assert_eq!(hand.num(PieceType::BISHOP), 2);

    let mut hand2: Hand = hand;
    assert_eq!(hand, hand2);
    assert_eq!(hand2.num(PieceType::LANCE), 2);
    assert_eq!(hand2.num(PieceType::GOLD), 3);
    assert_eq!(hand2.num(PieceType::BISHOP), 2);
    hand2.minus_one(PieceType::LANCE);
    assert!(hand != hand2);
}

#[test]
fn test_hand_is_equal_or_superior() {
    let mut hand = Hand(0);
    hand.set(PieceType::PAWN, 17);
    hand.set(PieceType::SILVER, 3);
    hand.set(PieceType::ROOK, 2);
    let mut hand2 = hand;
    assert_eq!(hand.is_equal_or_superior(hand2), true);
    assert_eq!(hand2.is_equal_or_superior(hand), true);
    hand2.minus_one(PieceType::PAWN);
    assert_eq!(hand.is_equal_or_superior(hand2), true);
    assert_eq!(hand2.is_equal_or_superior(hand), false);
    hand2.plus_one(PieceType::BISHOP);
    assert_eq!(hand.is_equal_or_superior(hand2), false);
    assert_eq!(hand2.is_equal_or_superior(hand), false);
}
