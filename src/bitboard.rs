use crate::types::*;
use std::fmt;
use std::ops::*;

#[derive(Copy)]
pub struct Bitboard {
    pub v: [u64; 2],
}

impl Clone for Bitboard {
    fn clone(&self) -> Bitboard {
        Bitboard { v: self.v }
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, other: Bitboard) -> Bitboard {
        Bitboard {
            v: [self.value(0) | other.value(0), self.value(1) | other.value(1)],
        }
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, other: Bitboard) -> Bitboard {
        Bitboard {
            v: [self.value(0) & other.value(0), self.value(1) & other.value(1)],
        }
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, other: Bitboard) -> Bitboard {
        Bitboard {
            v: [self.value(0) ^ other.value(0), self.value(1) ^ other.value(1)],
        }
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Bitboard) {
        self.v[0] = self.value(0) | other.value(0);
        self.v[1] = self.value(1) | other.value(1);
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Bitboard) {
        self.v[0] = self.value(0) & other.value(0);
        self.v[1] = self.value(1) & other.value(1);
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Bitboard) {
        self.v[0] = self.value(0) ^ other.value(0);
        self.v[1] = self.value(1) ^ other.value(1);
    }
}

impl Shr<i32> for Bitboard {
    type Output = Bitboard;

    fn shr(self, other: i32) -> Bitboard {
        Bitboard {
            v: [self.v[0] >> other, self.v[1] >> other],
        }
    }
}

impl Shl<i32> for Bitboard {
    type Output = Bitboard;

    fn shl(self, other: i32) -> Bitboard {
        Bitboard {
            v: [self.v[0] << other, self.v[1] << other],
        }
    }
}

impl ShrAssign<i32> for Bitboard {
    fn shr_assign(&mut self, other: i32) {
        self.v[0] = self.v[0] >> other;
        self.v[1] = self.v[1] >> other;
    }
}

impl ShlAssign<i32> for Bitboard {
    fn shl_assign(&mut self, other: i32) {
        self.v[0] = self.v[0] << other;
        self.v[1] = self.v[1] << other;
    }
}

impl PartialEq for Bitboard {
    fn eq(&self, other: &Bitboard) -> bool {
        self.v[0] == other.v[0] && self.v[1] == other.v[1]
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        Bitboard {
            v: [!self.value(0), !self.value(1)],
        }
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bitboard {{ v: [{}, {}] }}", self.v[0], self.v[1])
    }
}

impl Bitboard {
    pub fn set(&mut self, sq: Square) {
        *self |= Bitboard::square_mask(sq);
    }
    #[allow(dead_code)]
    pub fn clear(&mut self, sq: Square) {
        *self &= !Bitboard::square_mask(sq);
    }
    pub fn xor(&mut self, sq: Square) {
        *self ^= Bitboard::square_mask(sq);
    }
    pub fn merge(&self) -> u64 {
        self.value(0) | self.value(1)
    }
    pub fn count_ones(&self) -> u32 {
        self.value(0).count_ones() + self.value(1).count_ones()
    }
    #[allow(dead_code)]
    pub fn notand(self, other: Bitboard) -> Bitboard {
        (!self) & other
    }
    pub fn to_bool(self) -> bool {
        self.merge() != 0
    }
    pub fn and_to_bool(self, other: Bitboard) -> bool {
        (self & other).to_bool()
    }
    pub fn is_set(&self, sq: Square) -> bool {
        self.and_to_bool(Bitboard::square_mask(sq))
    }
    #[allow(dead_code)]
    pub fn print(self) {
        println!("{}", self.to_string());
    }
    fn pop_lsb_right_unchecked(&mut self) -> Square {
        let sq = Square(self.value(0).trailing_zeros() as i32);
        self.v[0] &= self.v[0] - 1;
        sq
    }
    fn pop_lsb_left_unchecked(&mut self) -> Square {
        let sq = Square((self.value(1).trailing_zeros() + 63) as i32);
        self.v[1] &= self.v[1] - 1;
        sq
    }
    fn lsb_right_unchecked(&self) -> Square {
        Square(self.value(0).trailing_zeros() as i32)
    }
    fn lsb_left_unchecked(&self) -> Square {
        Square((self.value(1).trailing_zeros() + 63) as i32)
    }
    pub fn pop_lsb_unchecked(&mut self) -> Square {
        if self.value(0) != 0 {
            return self.pop_lsb_right_unchecked();
        }
        self.pop_lsb_left_unchecked()
    }
    pub fn pop_lsb(&mut self) -> Option<Square> {
        if self.value(0) != 0 {
            return Some(self.pop_lsb_right_unchecked());
        }
        if self.value(1) != 0 {
            return Some(self.pop_lsb_left_unchecked());
        }
        None
    }
    pub fn lsb_unchecked(&self) -> Square {
        if self.value(0) != 0 {
            return self.lsb_right_unchecked();
        }
        self.lsb_left_unchecked()
    }
    pub fn value(&self, i: usize) -> u64 {
        debug_assert!(i < Square::NUM);
        unsafe { *self.v.get_unchecked(i) }
    }
    pub fn part(sq: Square) -> usize {
        (Square::SQ79.0 < sq.0) as usize
    }
    pub const ZERO: Bitboard = Bitboard { v: [0, 0] };
    pub const ALL: Bitboard = Bitboard {
        v: [0x7fff_ffff_ffff_ffff, 0x3ffff],
    };
    #[rustfmt::skip]
    const SQUARE_MASK: [Bitboard; Square::NUM] = [
        Bitboard { v: [1      ,       0] },
        Bitboard { v: [1 <<  1,       0] },
        Bitboard { v: [1 <<  2,       0] },
        Bitboard { v: [1 <<  3,       0] },
        Bitboard { v: [1 <<  4,       0] },
        Bitboard { v: [1 <<  5,       0] },
        Bitboard { v: [1 <<  6,       0] },
        Bitboard { v: [1 <<  7,       0] },
        Bitboard { v: [1 <<  8,       0] },
        Bitboard { v: [1 <<  9,       0] },
        Bitboard { v: [1 << 10,       0] },
        Bitboard { v: [1 << 11,       0] },
        Bitboard { v: [1 << 12,       0] },
        Bitboard { v: [1 << 13,       0] },
        Bitboard { v: [1 << 14,       0] },
        Bitboard { v: [1 << 15,       0] },
        Bitboard { v: [1 << 16,       0] },
        Bitboard { v: [1 << 17,       0] },
        Bitboard { v: [1 << 18,       0] },
        Bitboard { v: [1 << 19,       0] },
        Bitboard { v: [1 << 20,       0] },
        Bitboard { v: [1 << 21,       0] },
        Bitboard { v: [1 << 22,       0] },
        Bitboard { v: [1 << 23,       0] },
        Bitboard { v: [1 << 24,       0] },
        Bitboard { v: [1 << 25,       0] },
        Bitboard { v: [1 << 26,       0] },
        Bitboard { v: [1 << 27,       0] },
        Bitboard { v: [1 << 28,       0] },
        Bitboard { v: [1 << 29,       0] },
        Bitboard { v: [1 << 30,       0] },
        Bitboard { v: [1 << 31,       0] },
        Bitboard { v: [1 << 32,       0] },
        Bitboard { v: [1 << 33,       0] },
        Bitboard { v: [1 << 34,       0] },
        Bitboard { v: [1 << 35,       0] },
        Bitboard { v: [1 << 36,       0] },
        Bitboard { v: [1 << 37,       0] },
        Bitboard { v: [1 << 38,       0] },
        Bitboard { v: [1 << 39,       0] },
        Bitboard { v: [1 << 40,       0] },
        Bitboard { v: [1 << 41,       0] },
        Bitboard { v: [1 << 42,       0] },
        Bitboard { v: [1 << 43,       0] },
        Bitboard { v: [1 << 44,       0] },
        Bitboard { v: [1 << 45,       0] },
        Bitboard { v: [1 << 46,       0] },
        Bitboard { v: [1 << 47,       0] },
        Bitboard { v: [1 << 48,       0] },
        Bitboard { v: [1 << 49,       0] },
        Bitboard { v: [1 << 50,       0] },
        Bitboard { v: [1 << 51,       0] },
        Bitboard { v: [1 << 52,       0] },
        Bitboard { v: [1 << 53,       0] },
        Bitboard { v: [1 << 54,       0] },
        Bitboard { v: [1 << 55,       0] },
        Bitboard { v: [1 << 56,       0] },
        Bitboard { v: [1 << 57,       0] },
        Bitboard { v: [1 << 58,       0] },
        Bitboard { v: [1 << 59,       0] },
        Bitboard { v: [1 << 60,       0] },
        Bitboard { v: [1 << 61,       0] },
        Bitboard { v: [1 << 62,       0] },
        Bitboard { v: [0      , 1      ] },
        Bitboard { v: [0      , 1 <<  1] },
        Bitboard { v: [0      , 1 <<  2] },
        Bitboard { v: [0      , 1 <<  3] },
        Bitboard { v: [0      , 1 <<  4] },
        Bitboard { v: [0      , 1 <<  5] },
        Bitboard { v: [0      , 1 <<  6] },
        Bitboard { v: [0      , 1 <<  7] },
        Bitboard { v: [0      , 1 <<  8] },
        Bitboard { v: [0      , 1 <<  9] },
        Bitboard { v: [0      , 1 << 10] },
        Bitboard { v: [0      , 1 << 11] },
        Bitboard { v: [0      , 1 << 12] },
        Bitboard { v: [0      , 1 << 13] },
        Bitboard { v: [0      , 1 << 14] },
        Bitboard { v: [0      , 1 << 15] },
        Bitboard { v: [0      , 1 << 16] },
        Bitboard { v: [0      , 1 << 17] },
    ];
    #[rustfmt::skip]    const FILE1_MASK: Bitboard = Bitboard { v: [0x1ff           , 0         ] };
    #[rustfmt::skip]    const FILE2_MASK: Bitboard = Bitboard { v: [0x1ff << 9      , 0         ] };
    #[rustfmt::skip]    const FILE3_MASK: Bitboard = Bitboard { v: [0x1ff << (9 * 2), 0         ] };
    #[rustfmt::skip]    const FILE4_MASK: Bitboard = Bitboard { v: [0x1ff << (9 * 3), 0         ] };
    #[rustfmt::skip]    const FILE5_MASK: Bitboard = Bitboard { v: [0x1ff << (9 * 4), 0         ] };
    #[rustfmt::skip]    const FILE6_MASK: Bitboard = Bitboard { v: [0x1ff << (9 * 5), 0         ] };
    #[rustfmt::skip]    const FILE7_MASK: Bitboard = Bitboard { v: [0x1ff << (9 * 6), 0         ] };
    #[rustfmt::skip]    const FILE8_MASK: Bitboard = Bitboard { v: [0               , 0x1ff     ] };
    #[rustfmt::skip]    const FILE9_MASK: Bitboard = Bitboard { v: [0               , 0x1ff << 9] };

    #[rustfmt::skip]    const RANK1_MASK: Bitboard = Bitboard { v: [0x40_2010_0804_0201     , 0x201     ] };
    #[rustfmt::skip]    const RANK2_MASK: Bitboard = Bitboard { v: [0x40_2010_0804_0201 << 1, 0x201 << 1] };
    #[rustfmt::skip]    const RANK3_MASK: Bitboard = Bitboard { v: [0x40_2010_0804_0201 << 2, 0x201 << 2] };
    #[rustfmt::skip]    const RANK4_MASK: Bitboard = Bitboard { v: [0x40_2010_0804_0201 << 3, 0x201 << 3] };
    #[rustfmt::skip]    const RANK5_MASK: Bitboard = Bitboard { v: [0x40_2010_0804_0201 << 4, 0x201 << 4] };
    #[rustfmt::skip]    const RANK6_MASK: Bitboard = Bitboard { v: [0x40_2010_0804_0201 << 5, 0x201 << 5] };
    #[rustfmt::skip]    const RANK7_MASK: Bitboard = Bitboard { v: [0x40_2010_0804_0201 << 6, 0x201 << 6] };
    #[rustfmt::skip]    const RANK8_MASK: Bitboard = Bitboard { v: [0x40_2010_0804_0201 << 7, 0x201 << 7] };
    #[rustfmt::skip]    const RANK9_MASK: Bitboard = Bitboard { v: [0x40_2010_0804_0201 << 8, 0x201 << 8] };

    pub fn square_mask(sq: Square) -> Bitboard {
        debug_assert!(0 <= sq.0);
        debug_assert!(sq.0 < Square::NUM as i32);
        unsafe { *Bitboard::SQUARE_MASK.get_unchecked(sq.0 as usize) }
    }
    pub fn file_mask(file: File) -> Bitboard {
        match file {
            File::FILE1 => Bitboard::FILE1_MASK,
            File::FILE2 => Bitboard::FILE2_MASK,
            File::FILE3 => Bitboard::FILE3_MASK,
            File::FILE4 => Bitboard::FILE4_MASK,
            File::FILE5 => Bitboard::FILE5_MASK,
            File::FILE6 => Bitboard::FILE6_MASK,
            File::FILE7 => Bitboard::FILE7_MASK,
            File::FILE8 => Bitboard::FILE8_MASK,
            File::FILE9 => Bitboard::FILE9_MASK,
            _ => unreachable!(),
        }
    }
    pub fn rank_mask(rank: Rank) -> Bitboard {
        match rank {
            Rank::RANK1 => Bitboard::RANK1_MASK,
            Rank::RANK2 => Bitboard::RANK2_MASK,
            Rank::RANK3 => Bitboard::RANK3_MASK,
            Rank::RANK4 => Bitboard::RANK4_MASK,
            Rank::RANK5 => Bitboard::RANK5_MASK,
            Rank::RANK6 => Bitboard::RANK6_MASK,
            Rank::RANK7 => Bitboard::RANK7_MASK,
            Rank::RANK8 => Bitboard::RANK8_MASK,
            Rank::RANK9 => Bitboard::RANK9_MASK,
            _ => unreachable!(),
        }
    }
    const BLACK_FIELD: Bitboard = Bitboard {
        // bit layout
        // 11 1111111
        // 11 1111111
        // 11 1111111
        // 00 0000000
        // 00 0000000
        // 00 0000000
        // 00 0000000
        // 00 0000000
        // 00 0000000
        v: [0x7038_1c0e_0703_81c0, 0x3_81c0],
    };
    const WHITE_FIELD: Bitboard = Bitboard {
        // bit layout
        // 00 0000000
        // 00 0000000
        // 00 0000000
        // 00 0000000
        // 00 0000000
        // 00 0000000
        // 11 1111111
        // 11 1111111
        // 11 1111111
        v: [0x1c0_e070_381c_0e07, 0xe07],
    };
    pub fn opponent_field_mask(us: Color) -> Bitboard {
        match us {
            Color::BLACK => Bitboard::WHITE_FIELD,
            Color::WHITE => Bitboard::BLACK_FIELD,
            _ => unreachable!(),
        }
    }
    #[allow(dead_code)]
    pub fn in_front_mask(c: Color, r: Rank) -> Bitboard {
        debug_assert!(0 <= c.0 && c.0 < Color::NUM as i32);
        debug_assert!(0 <= r.0 && r.0 < Rank::NUM as i32);
        unsafe { *IN_FRONT_MASKS.get_unchecked(r.0 as usize).get_unchecked(c.0 as usize) }
    }
    pub fn between_mask(sq0: Square, sq1: Square) -> Bitboard {
        debug_assert!(0 <= sq0.0 && sq0.0 < Square::NUM as i32);
        debug_assert!(0 <= sq1.0 && sq1.0 < Square::NUM as i32);
        unsafe { *BETWEEN_MASK.get_unchecked(sq0.0 as usize).get_unchecked(sq1.0 as usize) }
    }
    pub fn proximity_check_mask(pc_checking: Piece, ksq_checked: Square) -> Bitboard {
        debug_assert!(0 <= pc_checking.0 && pc_checking.0 < Piece::NUM as i32);
        debug_assert!(0 <= ksq_checked.0 && ksq_checked.0 < Square::NUM as i32);
        unsafe {
            *PROXIMITY_CHECK_MASK
                .get_unchecked(pc_checking.0 as usize)
                .get_unchecked(ksq_checked.0 as usize)
        }
    }
}

impl Iterator for Bitboard {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        if self.to_bool() {
            Some(self.pop_lsb_unchecked())
        } else {
            None
        }
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = "".to_string();
        for rank in Rank::ALL_FROM_UPPER.iter() {
            for file in File::ALL_FROM_LEFT.iter() {
                let sq = Square::new(*file, *rank);
                s += if self.is_set(sq) { "1" } else { "0" };
            }
            s += "\n";
        }
        s += "\n";
        write!(f, "{}", s)
    }
}

static IN_FRONT_MASKS: once_cell::sync::Lazy<[[Bitboard; Color::NUM]; Rank::NUM]> = once_cell::sync::Lazy::new(|| {
    let mut bbss: [[Bitboard; Color::NUM]; Rank::NUM] = [[Bitboard::ZERO; Color::NUM]; Rank::NUM];
    for r in Rank::ALL.iter() {
        for c in Color::ALL.iter() {
            let rab = RankAsBlack::new(*c, *r);
            for r_tmp in Rank::ALL.iter() {
                if r_tmp.is_in_front_of(*c, rab) {
                    bbss[r.0 as usize][c.0 as usize] |= Bitboard::rank_mask(*r_tmp);
                }
            }
        }
    }
    bbss
});
static BETWEEN_MASK: once_cell::sync::Lazy<[[Bitboard; Square::NUM]; Square::NUM]> = once_cell::sync::Lazy::new(|| {
    let mut bbss: [[Bitboard; Square::NUM]; Square::NUM] = [[Bitboard::ZERO; Square::NUM]; Square::NUM];
    for sq0 in Square::ALL.iter() {
        for sq1 in Square::ALL.iter() {
            let occupied = Bitboard::square_mask(*sq0) | Bitboard::square_mask(*sq1);
            let deltas: Vec<Square> = match Relation::new(*sq0, *sq1) {
                Relation::MISC => vec![],
                Relation::FILE_NS | Relation::FILE_SN => vec![Square::DELTA_N, Square::DELTA_S],
                Relation::RANK_EW | Relation::RANK_WE => vec![Square::DELTA_E, Square::DELTA_W],
                Relation::DIAG_NESW | Relation::DIAG_SWNE => {
                    vec![Square::DELTA_NE, Square::DELTA_SW]
                }
                Relation::DIAG_NWSE | Relation::DIAG_SENW => {
                    vec![Square::DELTA_NW, Square::DELTA_SE]
                }
                _ => unreachable!(),
            };
            bbss[sq0.0 as usize][sq1.0 as usize] =
                sliding_attacks(&deltas, *sq0, &occupied) & sliding_attacks(&deltas, *sq1, &occupied);
        }
    }
    bbss
});
static PROXIMITY_CHECK_MASK: once_cell::sync::Lazy<[[Bitboard; Square::NUM]; Piece::NUM]> = once_cell::sync::Lazy::new(|| {
    let mut bbss: [[Bitboard; Square::NUM]; Piece::NUM] = [[Bitboard::ZERO; Square::NUM]; Piece::NUM];
    for &ksq in Square::ALL.iter() {
        for &pc in &[
            Piece::B_PAWN,
            Piece::W_PAWN,
            Piece::B_LANCE,
            Piece::W_LANCE,
            Piece::B_KNIGHT,
            Piece::W_KNIGHT,
            Piece::B_SILVER,
            Piece::W_SILVER,
            Piece::B_GOLD,
            Piece::W_GOLD,
            Piece::B_BISHOP,
            Piece::W_BISHOP,
            Piece::B_ROOK,
            Piece::W_ROOK,
            Piece::B_PRO_PAWN,
            Piece::W_PRO_PAWN,
            Piece::B_PRO_LANCE,
            Piece::W_PRO_LANCE,
            Piece::B_PRO_KNIGHT,
            Piece::W_PRO_KNIGHT,
            Piece::B_PRO_SILVER,
            Piece::W_PRO_SILVER,
            Piece::B_HORSE,
            Piece::W_HORSE,
            Piece::B_DRAGON,
            Piece::W_DRAGON,
        ] {
            let pt = PieceType::new(pc);
            let us = Color::new(pc);
            for &from in Square::ALL.iter() {
                if from == ksq {
                    continue;
                }
                let to_bb = ATTACK_TABLE.attack(pt, us, from, &Bitboard::square_mask(ksq));
                for to in to_bb {
                    // use Bitboard::ALL as occupied bitboard. We allow only proximity check.
                    let check_attack_bb = ATTACK_TABLE.attack(pt, us, to, &Bitboard::ALL);
                    if check_attack_bb.is_set(ksq) {
                        bbss[pc.0 as usize][ksq.0 as usize].set(from);
                        break;
                    }
                    let rank_from = Rank::new(from);
                    let rank_to = Rank::new(to);
                    if pt.is_promotable() && (rank_from.is_opponent_field(us) || rank_to.is_opponent_field(us)) {
                        // use Bitboard::ALL as occupied bitboard. We allow only proximity check.
                        let check_attack_bb = ATTACK_TABLE.attack(pt.to_promote(), us, to, &Bitboard::ALL);
                        if check_attack_bb.is_set(ksq) {
                            bbss[pc.0 as usize][ksq.0 as usize].set(from);
                            break;
                        }
                    }
                }
            }
        }
    }
    bbss
});

fn sliding_attacks(deltas: &[Square], sq: Square, occupied: &Bitboard) -> Bitboard {
    let mut bb = Bitboard::ZERO;
    for delta in deltas {
        let mut sq_prev = sq;
        let mut sq_opt = sq.checked_add(*delta);
        while let Some(sq_tmp) = sq_opt {
            if (File::new(sq_prev).0 - File::new(sq_tmp).0).abs() <= 1 && (Rank::new(sq_prev).0 - Rank::new(sq_tmp).0).abs() <= 1
            {
                bb.set(sq_tmp);
                if occupied.is_set(sq_tmp) {
                    break;
                }
                sq_prev = sq_tmp;
                sq_opt = sq_tmp.checked_add(*delta);
            } else {
                break;
            }
        }
    }
    bb
}

#[derive(Debug)]
pub struct Magic<'a> {
    mask: Bitboard,
    magic: u64,
    attacks: &'a [Bitboard],
    shift: u32,
}

impl<'a> Magic<'a> {
    fn attack_mask(deltas: &[Square], sq: Square) -> Bitboard {
        let occupied = Bitboard::ZERO;
        let mut bb = sliding_attacks(deltas, sq, &occupied);
        let file = File::new(sq);
        let rank = Rank::new(sq);
        if file != File::FILE1 {
            bb &= !Bitboard::FILE1_MASK;
        }
        if file != File::FILE9 {
            bb &= !Bitboard::FILE9_MASK;
        }
        if rank != Rank::RANK1 {
            bb &= !Bitboard::RANK1_MASK;
        }
        if rank != Rank::RANK9 {
            bb &= !Bitboard::RANK9_MASK;
        }
        bb
    }
    fn index_to_occupied(index: u32, num_of_bits: u32, mask: &Bitboard) -> Bitboard {
        let mut tmp_bb: Bitboard = *mask;
        let mut ret = Bitboard::ZERO;
        for i in 0..num_of_bits {
            let sq = tmp_bb.pop_lsb_unchecked();
            if (index & (1 << i)) != 0 {
                ret.set(sq);
            }
        }
        ret
    }

    fn occupied_to_index(occupied: &Bitboard, magic: u64, shift: u32) -> usize {
        (occupied.merge().wrapping_mul(magic) >> shift) as usize
    }

    pub fn attack(&self, occupied: &Bitboard) -> Bitboard {
        unsafe {
            *self
                .attacks
                .get_unchecked(((self.mask & *occupied).merge().wrapping_mul(self.magic) >> self.shift) as usize)
        }
    }

    pub fn pseudo_attack(&self) -> Bitboard {
        debug_assert!(!self.attacks.is_empty());
        unsafe { *self.attacks.get_unchecked(0) }
    }
}

pub struct LanceAttackTable([[[Bitboard; LanceAttackTable::MASK_TABLE_NUM as usize]; Color::NUM]; Square::NUM]);

impl LanceAttackTable {
    const MASK_BITS: u32 = (File::NUM - 2) as u32;
    const MASK_TABLE_NUM: usize = 1 << LanceAttackTable::MASK_BITS;
    #[rustfmt::skip]
    const SLIDE: [i32; Square::NUM] = [
        1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 ,
        10, 10, 10, 10, 10, 10, 10, 10, 10,
        19, 19, 19, 19, 19, 19, 19, 19, 19,
        28, 28, 28, 28, 28, 28, 28, 28, 28,
        37, 37, 37, 37, 37, 37, 37, 37, 37,
        46, 46, 46, 46, 46, 46, 46, 46, 46,
        55, 55, 55, 55, 55, 55, 55, 55, 55,
        1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 ,
        10, 10, 10, 10, 10, 10, 10, 10, 10,
    ];
    fn attack_mask(sq: Square) -> Bitboard {
        Bitboard::file_mask(File::new(sq)) & !(Bitboard::RANK1_MASK | Bitboard::RANK9_MASK)
    }
    fn index_to_occupied(index: usize, num_of_bits: u32, mask: &Bitboard) -> Bitboard {
        let mut tmp_bb: Bitboard = *mask;
        let mut ret = Bitboard::ZERO;
        for i in 0..num_of_bits {
            let sq = tmp_bb.pop_lsb_unchecked();
            if (index & (1 << i)) != 0 {
                ret.set(sq);
            }
        }
        ret
    }
    fn new() -> LanceAttackTable {
        let mut ret = LanceAttackTable([[[Bitboard::ZERO; Self::MASK_TABLE_NUM]; Color::NUM]; Square::NUM]);
        for sq in Square::ALL.iter() {
            for c in Color::ALL.iter() {
                let mask = Self::attack_mask(*sq);
                let deltas = if *c == Color::BLACK {
                    vec![Square::DELTA_N]
                } else {
                    vec![Square::DELTA_S]
                };
                for i in 0..(Self::MASK_TABLE_NUM) {
                    let occupied = Self::index_to_occupied(i, Self::MASK_BITS, &mask);
                    ret.0[sq.0 as usize][c.0 as usize][i as usize] = sliding_attacks(&deltas, *sq, &occupied);
                }
            }
        }
        ret
    }
    pub fn attack(&self, c: Color, sq: Square, occupied: &Bitboard) -> Bitboard {
        let part = Bitboard::part(sq);
        debug_assert!(0 <= sq.0 && (sq.0 as usize) < Self::SLIDE.len());
        let index =
            ((occupied.value(part) >> unsafe { Self::SLIDE.get_unchecked(sq.0 as usize) }) as usize) & (Self::MASK_TABLE_NUM - 1);
        debug_assert!(0 <= c.0 && (c.0 as usize) < self.0[0].len());
        debug_assert!(index < self.0[0][0].len());
        unsafe {
            *self
                .0
                .get_unchecked(sq.0 as usize)
                .get_unchecked(c.0 as usize)
                .get_unchecked(index)
        }
    }
    pub fn pseudo_attack(&self, c: Color, sq: Square) -> Bitboard {
        debug_assert!(0 <= sq.0 && (sq.0 as usize) < self.0.len());
        debug_assert!(0 <= c.0 && (c.0 as usize) < self.0[0].len());
        unsafe {
            *self
                .0
                .get_unchecked(sq.0 as usize)
                .get_unchecked(c.0 as usize)
                .get_unchecked(0)
        }
    }
}

pub struct MagicTable<'a> {
    magics: [Magic<'a>; Square::NUM],
    _attacks: Vec<Bitboard>,
}

impl<'a> MagicTable<'a> {
    fn new(table_num: usize, shifts: &[i8; Square::NUM], magic_nums: &[u64; Square::NUM], deltas: &[Square]) -> MagicTable<'a> {
        let mut attacks = vec![Bitboard::ZERO; table_num];
        let mut magics = std::mem::MaybeUninit::<[Magic<'a>; Square::NUM]>::uninit();
        let mut count = 0;
        for sq in Square::ALL.iter() {
            let mask = Magic::attack_mask(deltas, *sq);
            let slice_attacks = unsafe {
                let ptr = attacks.as_mut_ptr().add(count);
                std::slice::from_raw_parts_mut(ptr, 1 << (64 - shifts[sq.0 as usize]))
            };
            for index in 0..(1 << mask.count_ones()) {
                let occupied = Magic::index_to_occupied(index, mask.count_ones(), &mask);
                slice_attacks[Magic::occupied_to_index(&occupied, magic_nums[sq.0 as usize], shifts[sq.0 as usize] as u32)] =
                    sliding_attacks(deltas, *sq, &occupied);
            }
            count += slice_attacks.len();
            let tmp_magic: Magic = Magic {
                mask: Magic::attack_mask(deltas, *sq),
                magic: magic_nums[sq.0 as usize],
                attacks: slice_attacks,
                shift: shifts[sq.0 as usize] as u32,
            };
            unsafe {
                (magics.as_mut_ptr() as *mut Magic).add(sq.0 as usize).write(tmp_magic);
            }
        }
        debug_assert_eq!(table_num, count);
        MagicTable {
            magics: unsafe { magics.assume_init() },
            _attacks: attacks,
        }
    }

    pub fn magic(&self, sq: Square) -> &Magic {
        debug_assert!(0 <= sq.0 && (sq.0 as usize) < self.magics.len());
        unsafe { self.magics.get_unchecked(sq.0 as usize) }
    }
}

pub struct KingAttackTable([Bitboard; Square::NUM]);

impl KingAttackTable {
    fn new() -> KingAttackTable {
        let mut ret = KingAttackTable([Bitboard::ZERO; Square::NUM]);
        let deltas = [
            Square::DELTA_N,
            Square::DELTA_NE,
            Square::DELTA_E,
            Square::DELTA_SE,
            Square::DELTA_S,
            Square::DELTA_SW,
            Square::DELTA_W,
            Square::DELTA_NW,
        ];
        for sq in Square::ALL.iter() {
            for delta in deltas.iter() {
                if let Some(sq_tmp) = sq.checked_add(*delta) {
                    if (File::new(*sq).0 - File::new(sq_tmp).0).abs() <= 1 && (Rank::new(*sq).0 - Rank::new(sq_tmp).0).abs() <= 1
                    {
                        ret.0[sq.0 as usize].set(sq_tmp);
                    }
                }
            }
        }
        ret
    }
    pub fn attack(&self, sq: Square) -> Bitboard {
        debug_assert!(0 <= sq.0 && (sq.0 as usize) < self.0.len());
        unsafe { *self.0.get_unchecked(sq.0 as usize) }
    }
}

pub struct PieceAttackTable([[Bitboard; Color::NUM]; Square::NUM]);

impl PieceAttackTable {
    const BLACK_PAWN_DELTAS: &'static [Square] = &[Square::DELTA_N];
    const WHITE_PAWN_DELTAS: &'static [Square] = &[Square::DELTA_S];
    const BLACK_KNIGHT_DELTAS: &'static [Square] = &[Square::DELTA_NNE, Square::DELTA_NNW];
    const WHITE_KNIGHT_DELTAS: &'static [Square] = &[Square::DELTA_SSE, Square::DELTA_SSW];
    const BLACK_SILVER_DELTAS: &'static [Square] = &[
        Square::DELTA_N,
        Square::DELTA_NE,
        Square::DELTA_SE,
        Square::DELTA_SW,
        Square::DELTA_NW,
    ];
    const WHITE_SILVER_DELTAS: &'static [Square] = &[
        Square::DELTA_NE,
        Square::DELTA_SE,
        Square::DELTA_S,
        Square::DELTA_SW,
        Square::DELTA_NW,
    ];
    const BLACK_GOLD_DELTAS: &'static [Square] = &[
        Square::DELTA_N,
        Square::DELTA_NE,
        Square::DELTA_E,
        Square::DELTA_S,
        Square::DELTA_W,
        Square::DELTA_NW,
    ];
    const WHITE_GOLD_DELTAS: &'static [Square] = &[
        Square::DELTA_N,
        Square::DELTA_E,
        Square::DELTA_SE,
        Square::DELTA_S,
        Square::DELTA_SW,
        Square::DELTA_W,
    ];

    fn new(deltass: &[&[Square]; Color::NUM]) -> PieceAttackTable {
        let mut ret = PieceAttackTable([[Bitboard::ZERO; Color::NUM]; Square::NUM]);
        for c in Color::ALL.iter() {
            for sq in Square::ALL.iter() {
                let deltas = &deltass[c.0 as usize];
                for delta in deltas.iter() {
                    if let Some(sq_tmp) = sq.checked_add(*delta) {
                        if (File::new(*sq).0 - File::new(sq_tmp).0).abs() <= 1
                            && (Rank::new(*sq).0 - Rank::new(sq_tmp).0).abs() <= 2
                        {
                            ret.0[sq.0 as usize][c.0 as usize].set(sq_tmp);
                        }
                    }
                }
            }
        }
        ret
    }
    pub fn attack(&self, c: Color, sq: Square) -> Bitboard {
        debug_assert!(0 <= sq.0 && (sq.0 as usize) < self.0.len());
        debug_assert!(0 <= c.0 && (c.0 as usize) < self.0[0].len());
        unsafe { *self.0.get_unchecked(sq.0 as usize).get_unchecked(c.0 as usize) }
    }
}

pub struct AttackTable<'a> {
    pub pawn: PieceAttackTable,
    pub lance: LanceAttackTable,
    pub knight: PieceAttackTable,
    pub silver: PieceAttackTable,
    pub gold: PieceAttackTable,
    pub king: KingAttackTable,
    pub bishop: MagicTable<'a>,
    pub rook: MagicTable<'a>,
}

impl<'a> AttackTable<'a> {
    const BISHOP_DELTAS: [Square; 4] = [Square::DELTA_NE, Square::DELTA_SE, Square::DELTA_SW, Square::DELTA_NW];
    const ROOK_DELTAS: [Square; 4] = [Square::DELTA_N, Square::DELTA_E, Square::DELTA_S, Square::DELTA_W];

    const BISHOP_ATTACK_TABLE_NUM: usize = 20224;
    const ROOK_ATTACK_TABLE_NUM: usize = 512_000; // if using pext, 512000 -> 4951616
    #[rustfmt::skip]
    const ROOK_SHIFT_BITS: [i8; Square::NUM] = [
        50, 51, 51, 51, 51, 51, 51, 51, 50,
        51, 52, 52, 52, 52, 52, 52, 52, 50, // [17]: 51 -> 50
        51, 52, 52, 52, 52, 52, 52, 52, 51,
        51, 52, 52, 52, 52, 52, 52, 52, 51,
        51, 52, 52, 52, 52, 52, 52, 52, 51,
        51, 52, 52, 52, 52, 52, 52, 52, 50, // [53]: 51 -> 50
        51, 52, 52, 52, 52, 52, 52, 52, 51,
        51, 52, 52, 52, 52, 52, 52, 52, 51,
        50, 51, 51, 51, 51, 51, 51, 51, 50,
    ];
    #[rustfmt::skip]
    const BISHOP_SHIFT_BITS: [i8; Square::NUM] = [
        57, 58, 58, 58, 58, 58, 58, 58, 57,
        58, 58, 58, 58, 58, 58, 58, 58, 58,
        58, 58, 56, 56, 56, 56, 56, 58, 58,
        58, 58, 56, 54, 54, 54, 56, 58, 58,
        58, 58, 56, 54, 52, 54, 56, 58, 58,
        58, 58, 56, 54, 54, 54, 56, 58, 58,
        58, 58, 56, 56, 56, 56, 56, 58, 58,
        58, 58, 58, 58, 58, 58, 58, 58, 58,
        57, 58, 58, 58, 58, 58, 58, 58, 57,
    ];
    #[rustfmt::skip]
    const BISHOP_MAGICS: [u64; Square::NUM] = [
        0x2010_1042_c820_0428, 0x0000_8402_4038_0102, 0x8008_00c0_1810_8251,
        0x0082_4280_1030_1000, 0x0481_0082_0100_0040, 0x8081_0204_2088_0800,
        0x0000_8042_2211_0000, 0x0000_e283_0140_0850, 0x2010_2214_2080_0810,
        0x2600_0100_2880_1824, 0x0008_0481_0210_2002, 0x4000_2481_0024_0402,
        0x4920_0200_428a_2108, 0x0000_4609_0402_0844, 0x2001_4010_2083_0200,
        0x0000_0010_0900_8120, 0x4804_0640_0820_8004, 0x4406_0002_4030_0ca0,
        0x0222_0014_0080_3220, 0x0226_0684_0018_2094, 0x9520_8402_010d_0104,
        0x4000_8075_0010_8102, 0xc000_2000_8050_0500, 0x5211_0003_0403_8020,
        0x1108_1001_8040_0820, 0x1000_1280_a8a2_1040, 0x1000_0480_9408_a210,
        0x0202_3000_0204_1112, 0x0404_0a80_0046_0408, 0x0204_0200_2104_0201,
        0x0008_1200_1318_0404, 0xa284_0080_0d02_0104, 0x200c_2010_0060_4080,
        0x1082_0040_0010_9408, 0x1000_21c0_0c41_0408, 0x8808_2090_5004_c801,
        0x1054_0640_8000_4120, 0x030c_0a02_2400_1030, 0x0300_0601_0004_0821,
        0x0512_0080_1020_c006, 0x2100_0400_4280_2801, 0x0481_0008_2040_1002,
        0x4040_8a04_5000_0801, 0x0081_0104_2000_00a2, 0x0281_1021_0210_8408,
        0x0804_0200_4028_0021, 0x2420_4012_0022_0040, 0x0800_1014_4080_c402,
        0x0080_1044_0080_0002, 0x1009_0480_8040_0081, 0x1000_8200_0201_008c,
        0x0010_0010_0808_0009, 0x02a5_006b_8008_0004, 0xc628_8018_200c_2884,
        0x1081_0010_4200_a000, 0x0141_0020_3081_4048, 0x0200_2040_8001_0808,
        0x0200_0040_1392_2002, 0x2200_0000_2005_0815, 0x2011_0104_0004_0800,
        0x1020_0400_0422_0200, 0x0944_0201_0484_0081, 0x6080_a080_801c_044a,
        0x2088_4008_1100_8020, 0x000c_40aa_0420_8070, 0x4100_8004_4090_0220,
        0x0000_0000_4811_2050, 0x8182_00d0_6201_2a10, 0x0402_0084_0450_8302,
        0x0000_1000_2010_1002, 0x0020_0404_2050_4912, 0x0002_0040_0811_8814,
        0x1000_8106_5008_4024, 0x1002_a030_0240_8804, 0x2104_2948_0118_1420,
        0x0841_0802_4050_0812, 0x4406_0090_0000_4884, 0x0080_0820_0401_2412,
        0x0080_0908_8080_8183, 0x0300_1200_2040_0410, 0x021a_0901_0082_2002,
    ];
    #[rustfmt::skip]
    const ROOK_MAGICS: [u64; Square::NUM] = [
        0x0140_0004_0080_9300, 0x1320_0009_0200_0240, 0x0080_0191_0c00_8180,
        0x0040_0200_0440_1040, 0x0040_0100_00d0_1120, 0x0080_0480_2008_4050,
        0x0040_0040_0008_0228, 0x0040_0440_000a_2a0a, 0x0040_0031_0101_0102,
        0x80c4_2000_1210_8100, 0x4010_c002_0400_0c01, 0x0220_4001_0325_0002,
        0x0002_6002_0000_4001, 0x0040_2000_5240_0020, 0x0c00_1000_2002_0008,
        0x9080_2010_0020_0004, 0x2200_2010_0008_0004, 0x8080_4c00_2020_0191,
        0x0045_3830_0000_9100, 0x0030_0028_0002_0040, 0x0040_1040_0098_8084,
        0x0108_0010_0080_0415, 0x0014_0050_0040_0009, 0x0d21_0010_01c0_0045,
        0x00c0_0030_0020_0024, 0x0040_0030_0028_0004, 0x0040_0210_0009_1102,
        0x2008_a204_0800_0d00, 0x2000_1000_8401_0040, 0x0144_0800_0800_8001,
        0x5010_2400_1000_26a2, 0x1040_0200_0800_1010, 0x1200_2000_2800_5010,
        0x4280_0300_3002_0898, 0x0480_0814_1001_1004, 0x0340_0004_0800_110a,
        0x0010_1000_010c_0021, 0x0009_2108_0008_0082, 0x0610_0002_0004_00a7,
        0xa224_0800_9008_00c0, 0x9220_0820_0100_0801, 0x1040_0080_0114_0030,
        0x0040_0022_2004_0008, 0x0280_0012_4008_010c, 0x0040_0084_0494_0002,
        0x0040_0408_0001_0200, 0x0090_0008_0900_2100, 0x2800_0800_0100_0201,
        0x1400_0200_0100_0201, 0x0180_0810_1401_8004, 0x1100_0080_0040_0201,
        0x0080_0040_0020_0201, 0x0420_8000_1000_0201, 0x2841_c000_8020_0209,
        0x0120_0024_0104_0001, 0x0145_1000_0101_000b, 0x0040_0800_0080_8001,
        0x0834_0001_8804_8001, 0x4001_2100_0080_0205, 0x0488_9a80_0740_0201,
        0x2080_0440_8020_0062, 0x0080_0040_0286_1002, 0x0000_c008_4204_9024,
        0x8040_0002_0202_0011, 0x0040_0404_002c_0100, 0x2080_0282_0200_0102,
        0x8100_0408_0059_0224, 0x2040_0090_0480_0010, 0x0040_0450_0040_0408,
        0x2200_2400_2080_2008, 0x4080_0420_0220_0204, 0x0040_00b0_000a_00a2,
        0x000a_6000_0081_0100, 0x0014_1000_0d00_1180, 0x0002_2001_0100_1080,
        0x1000_2001_4104_e120, 0x2407_2001_0000_4810, 0x8014_4000_a084_5050,
        0x1000_2000_6003_0c18, 0x4004_2000_2001_0102, 0x0140_6000_2101_0302,
    ];

    pub fn attack(&self, pt: PieceType, c: Color, sq: Square, occupied: &Bitboard) -> Bitboard {
        match pt {
            PieceType::PAWN => self.pawn.attack(c, sq),
            PieceType::LANCE => self.lance.attack(c, sq, occupied),
            PieceType::KNIGHT => self.knight.attack(c, sq),
            PieceType::SILVER => self.silver.attack(c, sq),
            PieceType::BISHOP => self.bishop.magic(sq).attack(occupied),
            PieceType::ROOK => self.rook.magic(sq).attack(occupied),
            PieceType::GOLD | PieceType::PRO_PAWN | PieceType::PRO_LANCE | PieceType::PRO_KNIGHT | PieceType::PRO_SILVER => {
                self.gold.attack(c, sq)
            }
            PieceType::KING => self.king.attack(sq),
            PieceType::HORSE => self.bishop.magic(sq).attack(occupied) | self.king.attack(sq),
            PieceType::DRAGON => self.rook.magic(sq).attack(occupied) | self.king.attack(sq),
            _ => unreachable!(),
        }
    }
}

pub static ATTACK_TABLE: once_cell::sync::Lazy<AttackTable<'static>> = once_cell::sync::Lazy::new(|| AttackTable {
    pawn: PieceAttackTable::new(&[&PieceAttackTable::BLACK_PAWN_DELTAS, &PieceAttackTable::WHITE_PAWN_DELTAS]),
    lance: LanceAttackTable::new(),
    knight: PieceAttackTable::new(&[&PieceAttackTable::BLACK_KNIGHT_DELTAS, &PieceAttackTable::WHITE_KNIGHT_DELTAS]),
    silver: PieceAttackTable::new(&[&PieceAttackTable::BLACK_SILVER_DELTAS, &PieceAttackTable::WHITE_SILVER_DELTAS]),
    gold: PieceAttackTable::new(&[&PieceAttackTable::BLACK_GOLD_DELTAS, &PieceAttackTable::WHITE_GOLD_DELTAS]),
    king: KingAttackTable::new(),
    bishop: MagicTable::new(
        AttackTable::BISHOP_ATTACK_TABLE_NUM,
        &AttackTable::BISHOP_SHIFT_BITS,
        &AttackTable::BISHOP_MAGICS,
        &AttackTable::BISHOP_DELTAS,
    ),
    rook: MagicTable::new(
        AttackTable::ROOK_ATTACK_TABLE_NUM,
        &AttackTable::ROOK_SHIFT_BITS,
        &AttackTable::ROOK_MAGICS,
        &AttackTable::ROOK_DELTAS,
    ),
});

//#[test]
//fn test_bitboard_union() {
//    let bb1 = Bitboard { v: [6, 0] };
//    let bb2 = Bitboard {
//        m: simd::i32x4::new(1, 0, 2, 0),
//    };
//    let mut bb3 = bb1 | bb2;
//    let bb4 = Bitboard { v: [7, 3] };
//    bb3.set(Square::SQ81);
//    assert_eq!(bb3.value(0), bb4.value(0));
//    assert_eq!(bb3.value(1), bb4.value(1));
//}

#[test]
fn test_bitboard_eq() {
    let bb0 = Bitboard::ZERO;
    let mut bb1 = Bitboard::ZERO;
    assert_eq!(bb0 == bb1, true);
    assert_eq!(bb0 != bb1, false);
    bb1.set(Square::SQ13);
    assert_eq!(bb0 == bb1, false);
    assert_eq!(bb0 != bb1, true);
}

#[test]
fn test_bitboard_part() {
    assert_eq!(Bitboard::part(Square::SQ11), 0);
    assert_eq!(Bitboard::part(Square::SQ79), 0);
    assert_eq!(Bitboard::part(Square::SQ81), 1);
    assert_eq!(Bitboard::part(Square::SQ99), 1);
}

#[test]
fn test_sliding_attacks() {
    let v = vec![Square::DELTA_N, Square::DELTA_E, Square::DELTA_S, Square::DELTA_W];
    let mut occupied = Bitboard::ZERO;
    occupied.set(Square::SQ46);
    let bb = sliding_attacks(&v, Square::SQ44, &occupied);
    assert_eq!(bb.is_set(Square::SQ46), true);
    assert_eq!(bb.is_set(Square::SQ47), false);
    assert_eq!(bb.is_set(Square::SQ44), false);
    assert_eq!(bb.is_set(Square::SQ14), true);
    assert_eq!(bb.is_set(Square::SQ94), true);
    assert_eq!(bb.is_set(Square::SQ41), true);
    assert_eq!(bb.is_set(Square::SQ22), false);
    assert_eq!(bb.is_set(Square::SQ71), false);
    assert_eq!(bb.is_set(Square::SQ17), false);

    let v = vec![Square::DELTA_NE, Square::DELTA_SE, Square::DELTA_SW, Square::DELTA_NW];
    let mut occupied = Bitboard::ZERO;
    occupied.set(Square::SQ66);
    let bb = sliding_attacks(&v, Square::SQ44, &occupied);
    assert_eq!(bb.is_set(Square::SQ66), true);
    assert_eq!(bb.is_set(Square::SQ77), false);
    assert_eq!(bb.is_set(Square::SQ44), false);
    assert_eq!(bb.is_set(Square::SQ22), true);
    assert_eq!(bb.is_set(Square::SQ71), true);
    assert_eq!(bb.is_set(Square::SQ17), true);
    assert_eq!(bb.is_set(Square::SQ14), false);
    assert_eq!(bb.is_set(Square::SQ94), false);
    assert_eq!(bb.is_set(Square::SQ41), false);
}

#[test]
fn test_clone() {
    let bb = Bitboard::ZERO;
    let bb2 = bb;
    assert_eq!(bb, bb2);
}

#[test]
fn test_opponent_field_mask() {
    for us in Color::ALL.iter() {
        for sq in Square::ALL.iter() {
            let rank = Rank::new(*sq);
            assert_eq!(rank.is_opponent_field(*us), Bitboard::opponent_field_mask(*us).is_set(*sq));
        }
    }
}

#[test]
fn test_block_bits() {
    let v = vec![Square::DELTA_N, Square::DELTA_E, Square::DELTA_S, Square::DELTA_W];
    let mut occupied = Bitboard::ZERO;
    occupied.set(Square::SQ46);
    let bb = sliding_attacks(&v, Square::SQ11, &occupied);
    let bits = bb.count_ones();
    assert_eq!(bits, 16);
}

#[test]
fn test_bishop_magic() {
    std::thread::Builder::new()
        .stack_size(crate::stack_size::STACK_SIZE)
        .spawn(|| {
            let mut occupied = Bitboard::ZERO;
            occupied.set(Square::SQ66);
            occupied.set(Square::SQ33);
            occupied.set(Square::SQ22);
            occupied.set(Square::SQ82);
            occupied.set(Square::SQ28);
            assert_eq!(
                ATTACK_TABLE.bishop.magic(Square::SQ55).attack(&occupied),
                sliding_attacks(&AttackTable::BISHOP_DELTAS, Square::SQ55, &occupied)
            );
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn test_rook_magic() {
    let mut occupied = Bitboard::ZERO;
    occupied.set(Square::SQ65);
    occupied.set(Square::SQ35);
    occupied.set(Square::SQ25);
    occupied.set(Square::SQ52);
    occupied.set(Square::SQ58);
    assert_eq!(
        ATTACK_TABLE.rook.magic(Square::SQ55).attack(&occupied),
        sliding_attacks(&AttackTable::ROOK_DELTAS, Square::SQ55, &occupied)
    );
}

#[test]
fn test_lance_attack() {
    std::thread::Builder::new()
        .stack_size(crate::stack_size::STACK_SIZE)
        .spawn(|| {
            let mut occupied = Bitboard::ZERO;
            occupied.set(Square::SQ52);
            let attack = ATTACK_TABLE.lance.attack(Color::BLACK, Square::SQ55, &occupied);
            assert_eq!(attack.is_set(Square::SQ55), false);
            assert_eq!(attack.is_set(Square::SQ54), true);
            assert_eq!(attack.is_set(Square::SQ52), true);
            assert_eq!(attack.is_set(Square::SQ51), false);

            let mut occupied = Bitboard::ZERO;
            occupied.set(Square::SQ58);
            let attack = ATTACK_TABLE.lance.attack(Color::WHITE, Square::SQ55, &occupied);
            assert_eq!(attack.is_set(Square::SQ55), false);
            assert_eq!(attack.is_set(Square::SQ56), true);
            assert_eq!(attack.is_set(Square::SQ58), true);
            assert_eq!(attack.is_set(Square::SQ59), false);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn test_king_attack() {
    std::thread::Builder::new()
        .stack_size(crate::stack_size::STACK_SIZE)
        .spawn(|| {
            let mut bb = Bitboard::ZERO;
            bb.set(Square::SQ12);
            bb.set(Square::SQ21);
            bb.set(Square::SQ22);
            assert_eq!(ATTACK_TABLE.king.attack(Square::SQ11), bb);

            let mut bb = Bitboard::ZERO;
            bb.set(Square::SQ77);
            bb.set(Square::SQ78);
            bb.set(Square::SQ79);
            bb.set(Square::SQ87);
            bb.set(Square::SQ89);
            bb.set(Square::SQ97);
            bb.set(Square::SQ98);
            bb.set(Square::SQ99);
            assert_eq!(ATTACK_TABLE.king.attack(Square::SQ88), bb);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn test_piece_attack() {
    std::thread::Builder::new()
        .stack_size(crate::stack_size::STACK_SIZE)
        .spawn(|| {
            let bb = Bitboard::ZERO;
            assert_eq!(ATTACK_TABLE.pawn.attack(Color::BLACK, Square::SQ11), bb);

            let mut bb = Bitboard::ZERO;
            bb.set(Square::SQ12);
            assert_eq!(ATTACK_TABLE.pawn.attack(Color::WHITE, Square::SQ11), bb);

            let mut bb = Bitboard::ZERO;
            bb.set(Square::SQ77);
            bb.set(Square::SQ78);
            bb.set(Square::SQ87);
            bb.set(Square::SQ89);
            bb.set(Square::SQ97);
            bb.set(Square::SQ98);
            assert_eq!(ATTACK_TABLE.gold.attack(Color::BLACK, Square::SQ88), bb);

            let mut bb = Bitboard::ZERO;
            bb.set(Square::SQ77);
            bb.set(Square::SQ79);
            bb.set(Square::SQ87);
            bb.set(Square::SQ97);
            bb.set(Square::SQ99);
            assert_eq!(ATTACK_TABLE.silver.attack(Color::BLACK, Square::SQ88), bb);

            let mut bb = Bitboard::ZERO;
            bb.set(Square::SQ76);
            bb.set(Square::SQ96);
            assert_eq!(ATTACK_TABLE.knight.attack(Color::BLACK, Square::SQ88), bb);

            let bb = Bitboard::ZERO;
            assert_eq!(ATTACK_TABLE.knight.attack(Color::WHITE, Square::SQ88), bb);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn test_pseudo_attack() {
    for sq in Square::ALL.iter() {
        assert_eq!(
            ATTACK_TABLE.bishop.magic(*sq).attack(&Bitboard::ZERO),
            ATTACK_TABLE.bishop.magic(*sq).pseudo_attack()
        );
        assert_eq!(
            ATTACK_TABLE.rook.magic(*sq).attack(&Bitboard::ZERO),
            ATTACK_TABLE.rook.magic(*sq).pseudo_attack()
        );
    }
    for c in Color::ALL.iter() {
        for sq in Square::ALL.iter() {
            assert_eq!(
                ATTACK_TABLE.lance.attack(*c, *sq, &Bitboard::ZERO),
                ATTACK_TABLE.lance.pseudo_attack(*c, *sq)
            );
        }
    }
}

#[test]
fn test_in_front_mask() {
    for sq in Square::ALL.iter() {
        let rank = Rank::new(*sq);
        for c in Color::ALL.iter() {
            let rab = RankAsBlack::new(*c, rank);
            for sq_tmp in Square::ALL.iter() {
                let rank_tmp = Rank::new(*sq_tmp);
                assert_eq!(
                    rank_tmp.is_in_front_of(*c, rab),
                    Bitboard::in_front_mask(*c, rank).is_set(*sq_tmp)
                );
            }
        }
    }
}

#[test]
fn test_between_mask() {
    std::thread::Builder::new()
        .stack_size(crate::stack_size::STACK_SIZE)
        .spawn(|| {
            for sq0 in Square::ALL.iter() {
                for sq1 in Square::ALL.iter() {
                    let relation = Relation::new(*sq0, *sq1);
                    if relation == Relation::MISC {
                        assert_eq!(Bitboard::between_mask(*sq0, *sq1).count_ones(), 0);
                    } else {
                        let occupied = Bitboard::square_mask(*sq0) | Bitboard::square_mask(*sq1);
                        let attack;
                        if relation.is_cross() {
                            attack =
                                ATTACK_TABLE.rook.magic(*sq0).attack(&occupied) & ATTACK_TABLE.rook.magic(*sq1).attack(&occupied);
                        } else if relation.is_diag() {
                            attack = ATTACK_TABLE.bishop.magic(*sq0).attack(&occupied)
                                & ATTACK_TABLE.bishop.magic(*sq1).attack(&occupied);
                        } else {
                            unreachable!();
                        }
                        assert_eq!(Bitboard::between_mask(*sq0, *sq1), attack);
                    }
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn test_proximity_check_mask() {
    std::thread::Builder::new()
        .stack_size(crate::stack_size::STACK_SIZE)
        .spawn(|| {
            let check_candidates = Bitboard::proximity_check_mask(Piece::B_PAWN, Square::SQ53);
            assert_eq!(check_candidates.count_ones(), 3);
            assert!(check_candidates.is_set(Square::SQ44));
            assert!(check_candidates.is_set(Square::SQ55));
            assert!(check_candidates.is_set(Square::SQ64));

            let check_candidates = Bitboard::proximity_check_mask(Piece::B_BISHOP, Square::SQ53);
            assert!(check_candidates.is_set(Square::SQ55));
            assert!(!check_candidates.is_set(Square::SQ12));
            assert!(!check_candidates.is_set(Square::SQ57)); // not proximity.
        })
        .unwrap()
        .join()
        .unwrap();
}
