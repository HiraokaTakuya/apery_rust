use crate::bitboard::*;
use crate::piecevalue::*;
use serde::{Deserialize, Serialize};

pub struct True;
pub struct False;
pub trait Bool {
    const BOOL: bool;
}
impl Bool for True {
    const BOOL: bool = true;
}
impl Bool for False {
    const BOOL: bool = false;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Color(pub i32);

impl Color {
    pub const BLACK: Color = Color(0);
    pub const WHITE: Color = Color(1);
    pub const NUM: usize = 2;

    pub const ALL: [Color; Color::NUM] = [Color::BLACK, Color::WHITE];
    pub const ALL_FROM_BLACK: [Color; Color::NUM] = [Color::BLACK, Color::WHITE];

    pub fn inverse(self) -> Color {
        Color(1 ^ self.0)
    }
    pub fn new(pc: Piece) -> Color {
        Color((pc.0 & Piece::WHITE_BIT) >> Piece::WHITE_BIT_SHIFT)
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            Color::BLACK => "black",
            Color::WHITE => "white",
            _ => unreachable!(),
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct File(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rank(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(pub i32);

impl File {
    pub const FILE1: File = File(0);
    pub const FILE2: File = File(1);
    pub const FILE3: File = File(2);
    pub const FILE4: File = File(3);
    pub const FILE5: File = File(4);
    pub const FILE6: File = File(5);
    pub const FILE7: File = File(6);
    pub const FILE8: File = File(7);
    pub const FILE9: File = File(8);
    pub const NUM: usize = 9;

    pub const ALL_FROM_LEFT: [File; File::NUM] = [
        File::FILE9,
        File::FILE8,
        File::FILE7,
        File::FILE6,
        File::FILE5,
        File::FILE4,
        File::FILE3,
        File::FILE2,
        File::FILE1,
    ];
    #[allow(dead_code)]
    pub const ALL: [File; File::NUM] = [
        File::FILE1,
        File::FILE2,
        File::FILE3,
        File::FILE4,
        File::FILE5,
        File::FILE6,
        File::FILE7,
        File::FILE8,
        File::FILE9,
    ];

    #[rustfmt::skip]
    const SQUARE_TO_FILE: [File; Square::NUM] = [
        File::FILE1, File::FILE1, File::FILE1, File::FILE1, File::FILE1, File::FILE1, File::FILE1, File::FILE1, File::FILE1,
        File::FILE2, File::FILE2, File::FILE2, File::FILE2, File::FILE2, File::FILE2, File::FILE2, File::FILE2, File::FILE2,
        File::FILE3, File::FILE3, File::FILE3, File::FILE3, File::FILE3, File::FILE3, File::FILE3, File::FILE3, File::FILE3,
        File::FILE4, File::FILE4, File::FILE4, File::FILE4, File::FILE4, File::FILE4, File::FILE4, File::FILE4, File::FILE4,
        File::FILE5, File::FILE5, File::FILE5, File::FILE5, File::FILE5, File::FILE5, File::FILE5, File::FILE5, File::FILE5,
        File::FILE6, File::FILE6, File::FILE6, File::FILE6, File::FILE6, File::FILE6, File::FILE6, File::FILE6, File::FILE6,
        File::FILE7, File::FILE7, File::FILE7, File::FILE7, File::FILE7, File::FILE7, File::FILE7, File::FILE7, File::FILE7,
        File::FILE8, File::FILE8, File::FILE8, File::FILE8, File::FILE8, File::FILE8, File::FILE8, File::FILE8, File::FILE8,
        File::FILE9, File::FILE9, File::FILE9, File::FILE9, File::FILE9, File::FILE9, File::FILE9, File::FILE9, File::FILE9,
    ];

    pub fn new(sq: Square) -> File {
        debug_assert!(0 <= sq.0 && sq.0 <= Square::NUM as i32);
        unsafe { *File::SQUARE_TO_FILE.get_unchecked(sq.0 as usize) }
    }
    pub fn inverse(self) -> File {
        File(File::NUM as i32 - 1 - self.0)
    }
    pub fn to_usi_char(self) -> char {
        match self {
            File::FILE1 => '1',
            File::FILE2 => '2',
            File::FILE3 => '3',
            File::FILE4 => '4',
            File::FILE5 => '5',
            File::FILE6 => '6',
            File::FILE7 => '7',
            File::FILE8 => '8',
            File::FILE9 => '9',
            _ => unreachable!(),
        }
    }
    pub fn new_from_usi_char(c: char) -> Option<File> {
        match c {
            '1' => Some(File::FILE1),
            '2' => Some(File::FILE2),
            '3' => Some(File::FILE3),
            '4' => Some(File::FILE4),
            '5' => Some(File::FILE5),
            '6' => Some(File::FILE6),
            '7' => Some(File::FILE7),
            '8' => Some(File::FILE8),
            '9' => Some(File::FILE9),
            _ => None,
        }
    }
    pub fn to_csa_char(self) -> char {
        match self {
            File::FILE1 => '1',
            File::FILE2 => '2',
            File::FILE3 => '3',
            File::FILE4 => '4',
            File::FILE5 => '5',
            File::FILE6 => '6',
            File::FILE7 => '7',
            File::FILE8 => '8',
            File::FILE9 => '9',
            _ => unreachable!(),
        }
    }
    pub fn new_from_csa_char(c: char) -> Option<File> {
        match c {
            '1' => Some(File::FILE1),
            '2' => Some(File::FILE2),
            '3' => Some(File::FILE3),
            '4' => Some(File::FILE4),
            '5' => Some(File::FILE5),
            '6' => Some(File::FILE6),
            '7' => Some(File::FILE7),
            '8' => Some(File::FILE8),
            '9' => Some(File::FILE9),
            _ => None,
        }
    }
}

impl Rank {
    pub const RANK1: Rank = Rank(0);
    pub const RANK2: Rank = Rank(1);
    pub const RANK3: Rank = Rank(2);
    pub const RANK4: Rank = Rank(3);
    pub const RANK5: Rank = Rank(4);
    pub const RANK6: Rank = Rank(5);
    pub const RANK7: Rank = Rank(6);
    pub const RANK8: Rank = Rank(7);
    pub const RANK9: Rank = Rank(8);
    pub const NUM: usize = 9;

    pub const ALL_FROM_UPPER: [Rank; Rank::NUM] = [
        Rank::RANK1,
        Rank::RANK2,
        Rank::RANK3,
        Rank::RANK4,
        Rank::RANK5,
        Rank::RANK6,
        Rank::RANK7,
        Rank::RANK8,
        Rank::RANK9,
    ];
    pub const ALL: [Rank; Rank::NUM] = [
        Rank::RANK1,
        Rank::RANK2,
        Rank::RANK3,
        Rank::RANK4,
        Rank::RANK5,
        Rank::RANK6,
        Rank::RANK7,
        Rank::RANK8,
        Rank::RANK9,
    ];

    #[rustfmt::skip]
    const SQUARE_TO_RANK: [Rank; Square::NUM] = [
        Rank::RANK1, Rank::RANK2, Rank::RANK3, Rank::RANK4, Rank::RANK5, Rank::RANK6, Rank::RANK7, Rank::RANK8, Rank::RANK9,
        Rank::RANK1, Rank::RANK2, Rank::RANK3, Rank::RANK4, Rank::RANK5, Rank::RANK6, Rank::RANK7, Rank::RANK8, Rank::RANK9,
        Rank::RANK1, Rank::RANK2, Rank::RANK3, Rank::RANK4, Rank::RANK5, Rank::RANK6, Rank::RANK7, Rank::RANK8, Rank::RANK9,
        Rank::RANK1, Rank::RANK2, Rank::RANK3, Rank::RANK4, Rank::RANK5, Rank::RANK6, Rank::RANK7, Rank::RANK8, Rank::RANK9,
        Rank::RANK1, Rank::RANK2, Rank::RANK3, Rank::RANK4, Rank::RANK5, Rank::RANK6, Rank::RANK7, Rank::RANK8, Rank::RANK9,
        Rank::RANK1, Rank::RANK2, Rank::RANK3, Rank::RANK4, Rank::RANK5, Rank::RANK6, Rank::RANK7, Rank::RANK8, Rank::RANK9,
        Rank::RANK1, Rank::RANK2, Rank::RANK3, Rank::RANK4, Rank::RANK5, Rank::RANK6, Rank::RANK7, Rank::RANK8, Rank::RANK9,
        Rank::RANK1, Rank::RANK2, Rank::RANK3, Rank::RANK4, Rank::RANK5, Rank::RANK6, Rank::RANK7, Rank::RANK8, Rank::RANK9,
        Rank::RANK1, Rank::RANK2, Rank::RANK3, Rank::RANK4, Rank::RANK5, Rank::RANK6, Rank::RANK7, Rank::RANK8, Rank::RANK9,
    ];

    pub fn new(sq: Square) -> Rank {
        debug_assert!(0 <= sq.0 && sq.0 <= Square::NUM as i32);
        unsafe { *Rank::SQUARE_TO_RANK.get_unchecked(sq.0 as usize) }
    }
    pub fn new_from_color_and_rank_as_black(c: Color, rank_as_black: RankAsBlack) -> Rank {
        match c {
            Color::BLACK => Rank(rank_as_black.0),
            Color::WHITE => Rank(rank_as_black.0).inverse(),
            _ => unreachable!(),
        }
    }
    pub fn inverse(self) -> Rank {
        Rank(Rank::NUM as i32 - 1 - self.0)
    }
    pub fn new_from_usi_char(c: char) -> Option<Rank> {
        match c {
            'a' => Some(Rank::RANK1),
            'b' => Some(Rank::RANK2),
            'c' => Some(Rank::RANK3),
            'd' => Some(Rank::RANK4),
            'e' => Some(Rank::RANK5),
            'f' => Some(Rank::RANK6),
            'g' => Some(Rank::RANK7),
            'h' => Some(Rank::RANK8),
            'i' => Some(Rank::RANK9),
            _ => None,
        }
    }
    pub fn new_from_csa_char(c: char) -> Option<Rank> {
        match c {
            '1' => Some(Rank::RANK1),
            '2' => Some(Rank::RANK2),
            '3' => Some(Rank::RANK3),
            '4' => Some(Rank::RANK4),
            '5' => Some(Rank::RANK5),
            '6' => Some(Rank::RANK6),
            '7' => Some(Rank::RANK7),
            '8' => Some(Rank::RANK8),
            '9' => Some(Rank::RANK9),
            _ => None,
        }
    }
    pub fn to_usi_char(self) -> char {
        match self {
            Rank::RANK1 => 'a',
            Rank::RANK2 => 'b',
            Rank::RANK3 => 'c',
            Rank::RANK4 => 'd',
            Rank::RANK5 => 'e',
            Rank::RANK6 => 'f',
            Rank::RANK7 => 'g',
            Rank::RANK8 => 'h',
            Rank::RANK9 => 'i',
            _ => unreachable!(),
        }
    }
    pub fn to_csa_char(self) -> char {
        match self {
            Rank::RANK1 => '1',
            Rank::RANK2 => '2',
            Rank::RANK3 => '3',
            Rank::RANK4 => '4',
            Rank::RANK5 => '5',
            Rank::RANK6 => '6',
            Rank::RANK7 => '7',
            Rank::RANK8 => '8',
            Rank::RANK9 => '9',
            _ => unreachable!(),
        }
    }
    pub fn is_opponent_field(self, us: Color) -> bool {
        (0x1c0_0007 & (1 << ((us.0 << 4) + self.0))) != 0
    }
    pub fn is_in_front_of(self, us: Color, rank_as_black: RankAsBlack) -> bool {
        const_assert!(File::FILE1.0 < File::FILE9.0);
        match us {
            Color::BLACK => self.0 < Rank::new_from_color_and_rank_as_black(Color::BLACK, rank_as_black).0,
            Color::WHITE => self.0 > Rank::new_from_color_and_rank_as_black(Color::WHITE, rank_as_black).0,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RankAsBlack(i32);

impl RankAsBlack {
    #[allow(dead_code)]
    pub const RANK1: RankAsBlack = RankAsBlack(0);
    #[allow(dead_code)]
    pub const RANK2: RankAsBlack = RankAsBlack(1);
    #[allow(dead_code)]
    pub const RANK3: RankAsBlack = RankAsBlack(2);
    #[allow(dead_code)]
    pub const RANK4: RankAsBlack = RankAsBlack(3);
    #[allow(dead_code)]
    pub const RANK5: RankAsBlack = RankAsBlack(4);
    #[allow(dead_code)]
    pub const RANK6: RankAsBlack = RankAsBlack(5);
    #[allow(dead_code)]
    pub const RANK7: RankAsBlack = RankAsBlack(6);
    #[allow(dead_code)]
    pub const RANK8: RankAsBlack = RankAsBlack(7);
    #[allow(dead_code)]
    pub const RANK9: RankAsBlack = RankAsBlack(8);
    const NUM: usize = Rank::NUM;

    #[allow(dead_code)]
    pub const ALL: [RankAsBlack; RankAsBlack::NUM] = [
        RankAsBlack::RANK1,
        RankAsBlack::RANK2,
        RankAsBlack::RANK3,
        RankAsBlack::RANK4,
        RankAsBlack::RANK5,
        RankAsBlack::RANK6,
        RankAsBlack::RANK7,
        RankAsBlack::RANK8,
        RankAsBlack::RANK9,
    ];
    pub fn new(c: Color, r: Rank) -> RankAsBlack {
        match c {
            Color::BLACK => RankAsBlack(r.0),
            Color::WHITE => RankAsBlack(r.inverse().0),
            _ => unreachable!(),
        }
    }
}

impl Square {
    pub const SQ11: Square = Square(0);
    pub const SQ12: Square = Square(1);
    pub const SQ13: Square = Square(2);
    pub const SQ14: Square = Square(3);
    pub const SQ15: Square = Square(4);
    pub const SQ16: Square = Square(5);
    pub const SQ17: Square = Square(6);
    pub const SQ18: Square = Square(7);
    pub const SQ19: Square = Square(8);
    pub const SQ21: Square = Square(9);
    pub const SQ22: Square = Square(10);
    pub const SQ23: Square = Square(11);
    pub const SQ24: Square = Square(12);
    pub const SQ25: Square = Square(13);
    pub const SQ26: Square = Square(14);
    pub const SQ27: Square = Square(15);
    pub const SQ28: Square = Square(16);
    pub const SQ29: Square = Square(17);
    pub const SQ31: Square = Square(18);
    pub const SQ32: Square = Square(19);
    pub const SQ33: Square = Square(20);
    pub const SQ34: Square = Square(21);
    pub const SQ35: Square = Square(22);
    pub const SQ36: Square = Square(23);
    pub const SQ37: Square = Square(24);
    pub const SQ38: Square = Square(25);
    pub const SQ39: Square = Square(26);
    pub const SQ41: Square = Square(27);
    pub const SQ42: Square = Square(28);
    pub const SQ43: Square = Square(29);
    pub const SQ44: Square = Square(30);
    pub const SQ45: Square = Square(31);
    pub const SQ46: Square = Square(32);
    pub const SQ47: Square = Square(33);
    pub const SQ48: Square = Square(34);
    pub const SQ49: Square = Square(35);
    pub const SQ51: Square = Square(36);
    pub const SQ52: Square = Square(37);
    pub const SQ53: Square = Square(38);
    pub const SQ54: Square = Square(39);
    pub const SQ55: Square = Square(40);
    pub const SQ56: Square = Square(41);
    pub const SQ57: Square = Square(42);
    pub const SQ58: Square = Square(43);
    pub const SQ59: Square = Square(44);
    pub const SQ61: Square = Square(45);
    pub const SQ62: Square = Square(46);
    pub const SQ63: Square = Square(47);
    pub const SQ64: Square = Square(48);
    pub const SQ65: Square = Square(49);
    pub const SQ66: Square = Square(50);
    pub const SQ67: Square = Square(51);
    pub const SQ68: Square = Square(52);
    pub const SQ69: Square = Square(53);
    pub const SQ71: Square = Square(54);
    pub const SQ72: Square = Square(55);
    pub const SQ73: Square = Square(56);
    pub const SQ74: Square = Square(57);
    pub const SQ75: Square = Square(58);
    pub const SQ76: Square = Square(59);
    pub const SQ77: Square = Square(60);
    pub const SQ78: Square = Square(61);
    pub const SQ79: Square = Square(62);
    pub const SQ81: Square = Square(63);
    pub const SQ82: Square = Square(64);
    pub const SQ83: Square = Square(65);
    pub const SQ84: Square = Square(66);
    pub const SQ85: Square = Square(67);
    pub const SQ86: Square = Square(68);
    pub const SQ87: Square = Square(69);
    pub const SQ88: Square = Square(70);
    pub const SQ89: Square = Square(71);
    pub const SQ91: Square = Square(72);
    pub const SQ92: Square = Square(73);
    pub const SQ93: Square = Square(74);
    pub const SQ94: Square = Square(75);
    pub const SQ95: Square = Square(76);
    pub const SQ96: Square = Square(77);
    pub const SQ97: Square = Square(78);
    pub const SQ98: Square = Square(79);
    pub const SQ99: Square = Square(80);
    pub const NUM: usize = 81;
    pub const DELTA_N: Square = Square(-1);
    pub const DELTA_E: Square = Square(-(File::NUM as i32));
    pub const DELTA_S: Square = Square(1);
    pub const DELTA_W: Square = Square(File::NUM as i32);
    pub const DELTA_NE: Square = Square(Square::DELTA_N.0 + Square::DELTA_E.0);
    pub const DELTA_SE: Square = Square(Square::DELTA_S.0 + Square::DELTA_E.0);
    pub const DELTA_SW: Square = Square(Square::DELTA_S.0 + Square::DELTA_W.0);
    pub const DELTA_NW: Square = Square(Square::DELTA_N.0 + Square::DELTA_W.0);
    pub const DELTA_NNE: Square = Square(Square::DELTA_N.0 + Square::DELTA_NE.0);
    pub const DELTA_SSE: Square = Square(Square::DELTA_S.0 + Square::DELTA_SE.0);
    pub const DELTA_SSW: Square = Square(Square::DELTA_S.0 + Square::DELTA_SW.0);
    pub const DELTA_NNW: Square = Square(Square::DELTA_N.0 + Square::DELTA_NW.0);

    pub const ALL: [Square; Square::NUM] = [
        Square::SQ11,
        Square::SQ12,
        Square::SQ13,
        Square::SQ14,
        Square::SQ15,
        Square::SQ16,
        Square::SQ17,
        Square::SQ18,
        Square::SQ19,
        Square::SQ21,
        Square::SQ22,
        Square::SQ23,
        Square::SQ24,
        Square::SQ25,
        Square::SQ26,
        Square::SQ27,
        Square::SQ28,
        Square::SQ29,
        Square::SQ31,
        Square::SQ32,
        Square::SQ33,
        Square::SQ34,
        Square::SQ35,
        Square::SQ36,
        Square::SQ37,
        Square::SQ38,
        Square::SQ39,
        Square::SQ41,
        Square::SQ42,
        Square::SQ43,
        Square::SQ44,
        Square::SQ45,
        Square::SQ46,
        Square::SQ47,
        Square::SQ48,
        Square::SQ49,
        Square::SQ51,
        Square::SQ52,
        Square::SQ53,
        Square::SQ54,
        Square::SQ55,
        Square::SQ56,
        Square::SQ57,
        Square::SQ58,
        Square::SQ59,
        Square::SQ61,
        Square::SQ62,
        Square::SQ63,
        Square::SQ64,
        Square::SQ65,
        Square::SQ66,
        Square::SQ67,
        Square::SQ68,
        Square::SQ69,
        Square::SQ71,
        Square::SQ72,
        Square::SQ73,
        Square::SQ74,
        Square::SQ75,
        Square::SQ76,
        Square::SQ77,
        Square::SQ78,
        Square::SQ79,
        Square::SQ81,
        Square::SQ82,
        Square::SQ83,
        Square::SQ84,
        Square::SQ85,
        Square::SQ86,
        Square::SQ87,
        Square::SQ88,
        Square::SQ89,
        Square::SQ91,
        Square::SQ92,
        Square::SQ93,
        Square::SQ94,
        Square::SQ95,
        Square::SQ96,
        Square::SQ97,
        Square::SQ98,
        Square::SQ99,
    ];

    pub fn new(f: File, r: Rank) -> Square {
        Square(f.0 * 9 + r.0)
    }
    pub fn inverse(self) -> Square {
        Square(Square::NUM as i32 - 1 - self.0)
    }
    #[allow(dead_code)]
    pub fn inverse_file(self) -> Square {
        Square::new(File::new(self).inverse(), Rank::new(self))
    }
    pub fn to_usi_string(self) -> String {
        let v = [File::new(self).to_usi_char(), Rank::new(self).to_usi_char()];
        let s: String = v.iter().collect();
        s
    }
    #[allow(dead_code)]
    pub fn to_csa_string(self) -> String {
        let v = [File::new(self).to_csa_char(), Rank::new(self).to_csa_char()];
        let s: String = v.iter().collect();
        s
    }
    pub fn is_ok(self) -> bool {
        0 <= self.0 && self.0 < Square::NUM as i32
    }
    pub fn checked_add(self, delta: Square) -> Option<Square> {
        let sq = Square(self.0 + delta.0);
        if sq.is_ok() {
            return Some(sq);
        }
        None
    }
    pub fn add_unchecked(self, delta: Square) -> Square {
        Square(self.0 + delta.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Relation(pub u16);

impl Relation {
    pub const MISC: Relation = Relation(0b0000_0000_0000_0001);
    pub const FILE_SN: Relation = Relation(0b0000_0000_0000_0010);
    pub const FILE_NS: Relation = Relation(0b0000_0000_0000_0100);
    pub const RANK_EW: Relation = Relation(0b0000_0000_0000_1000);
    pub const RANK_WE: Relation = Relation(0b0000_0000_0001_0000);
    pub const DIAG_NESW: Relation = Relation(0b0000_0000_0010_0000);
    pub const DIAG_SENW: Relation = Relation(0b0000_0000_0100_0000);
    pub const DIAG_SWNE: Relation = Relation(0b0000_0000_1000_0000);
    pub const DIAG_NWSE: Relation = Relation(0b0000_0001_0000_0000);

    const FILE: Relation = Relation(0b0000_0000_0000_0110);
    const RANK: Relation = Relation(0b0000_0000_0001_1000);

    const DIAG_NE: Relation = Relation(0b0000_0000_1010_0000);
    const DIAG_SE: Relation = Relation(0b0000_0001_0100_0000);

    const CROSS: Relation = Relation(0b0000_0000_0001_1110);
    const DIAG: Relation = Relation(0b0000_0001_1110_0000);

    pub fn new(from: Square, to: Square) -> Relation {
        debug_assert!(from.is_ok());
        debug_assert!(to.is_ok());
        unsafe { *RELATION_TABLE.0.get_unchecked(from.0 as usize).get_unchecked(to.0 as usize) }
    }
    #[allow(dead_code)]
    pub fn is_file(self) -> bool {
        (self.0 & Self::FILE.0) != 0
    }
    #[allow(dead_code)]
    pub fn is_rank(self) -> bool {
        (self.0 & Self::RANK.0) != 0
    }
    #[allow(dead_code)]
    pub fn is_diag_ne(self) -> bool {
        (self.0 & Self::DIAG_NE.0) != 0
    }
    #[allow(dead_code)]
    pub fn is_diag_se(self) -> bool {
        (self.0 & Self::DIAG_SE.0) != 0
    }
    #[allow(dead_code)]
    pub fn is_cross(self) -> bool {
        (self.0 & Self::CROSS.0) != 0
    }
    pub fn is_diag(self) -> bool {
        (self.0 & Self::DIAG.0) != 0
    }
}

struct RelationTable([[Relation; Square::NUM]; Square::NUM]);

impl RelationTable {
    fn new() -> Self {
        let mut value: [[Relation; Square::NUM]; Square::NUM] = [[Relation::MISC; Square::NUM]; Square::NUM];
        for &from in Square::ALL.iter() {
            let f_from = File::new(from);
            let r_from = Rank::new(from);
            for &to in Square::ALL.iter() {
                value[from.0 as usize][to.0 as usize] = Relation::MISC;
                let f_to = File::new(to);
                let r_to = Rank::new(to);
                if from == to {
                    continue;
                }
                if f_from == f_to {
                    const_assert!(Rank::RANK1.0 < Rank::RANK2.0);
                    value[from.0 as usize][to.0 as usize] = if r_from.0 < r_to.0 {
                        Relation::FILE_NS
                    } else {
                        Relation::FILE_SN
                    };
                } else if r_from == r_to {
                    const_assert!(File::FILE1.0 < File::FILE2.0);
                    value[from.0 as usize][to.0 as usize] = if f_from.0 < f_to.0 {
                        Relation::RANK_EW
                    } else {
                        Relation::RANK_WE
                    };
                } else if r_from.0 - r_to.0 == f_from.0 - f_to.0 {
                    const_assert!(Rank::RANK1.0 < Rank::RANK2.0);
                    value[from.0 as usize][to.0 as usize] = if r_from.0 < r_to.0 {
                        Relation::DIAG_NESW
                    } else {
                        Relation::DIAG_SWNE
                    };
                } else if r_from.0 - r_to.0 == f_to.0 - f_from.0 {
                    const_assert!(Rank::RANK1.0 < Rank::RANK2.0);
                    value[from.0 as usize][to.0 as usize] = if r_from.0 < r_to.0 {
                        Relation::DIAG_NWSE
                    } else {
                        Relation::DIAG_SENW
                    };
                }
            }
        }
        RelationTable(value)
    }
}

static RELATION_TABLE: once_cell::sync::Lazy<RelationTable> = once_cell::sync::Lazy::new(RelationTable::new);

pub fn is_aligned_and_sq2_is_not_between_sq0_and_sq1(sq0: Square, sq1: Square, sq2: Square) -> bool {
    let relation_sq0_sq2 = Relation::new(sq0, sq2);
    let result = relation_sq0_sq2 != Relation::MISC && relation_sq0_sq2 == Relation::new(sq1, sq2);
    debug_assert!(result == (Bitboard::between_mask(sq0, sq2).is_set(sq1) || Bitboard::between_mask(sq1, sq2).is_set(sq0)));
    result
}

#[derive(PartialEq, Eq)]
pub struct Bound(pub i32);

impl Bound {
    pub const BOUND_NONE: Bound = Bound(0);
    pub const UPPER: Bound = Bound(1);
    pub const LOWER: Bound = Bound(2);
    pub const EXACT: Bound = Bound(Bound::UPPER.0 | Bound::LOWER.0);

    pub fn include_lower(&self) -> bool {
        (self.0 & Bound::LOWER.0) != 0
    }
    pub fn include_upper(&self) -> bool {
        (self.0 & Bound::UPPER.0) != 0
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Not,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    PartialOrd,
    Ord,
)]
pub struct Value(pub i32);

impl Value {
    pub const ZERO: Value = Value(0);
    pub const DRAW: Value = Value(0);
    pub const KNOWN_WIN: Value = Value(10000);
    #[allow(dead_code)]
    pub const MAX_EVALUATE: Value = Value(30000);
    const MATE_VAL: i32 = 32600;
    pub const MATE: Value = Value(Value::MATE_VAL);
    pub const MATED: Value = Value(-Value::MATE_VAL);
    pub const MATE_IN_MAX_PLY: Value = Value(Value::MATE_VAL - MAX_PLY);
    pub const MATED_IN_MAX_PLY: Value = Value(-Value::MATE_VAL + MAX_PLY);
    pub const INFINITE: Value = Value(32601);
    pub const NONE: Value = Value(32602);

    pub fn to_usi(self) -> String {
        if Value::MATED_IN_MAX_PLY < self && self < Value::MATE_IN_MAX_PLY {
            format!("cp {}", self.0 * 100 / PAWN_VALUE)
        } else {
            format!(
                "mate {}",
                if Value::ZERO < self {
                    Value::MATE.0 - self.0 + 1
                } else {
                    Value::MATED.0 - self.0
                }
            )
        }
    }
    #[allow(dead_code)]
    pub fn to_win_rate(self) -> f64 {
        if Value::MATED_IN_MAX_PLY < self && self < Value::MATE_IN_MAX_PLY {
            1.0 / (1.0 + (f64::from(-self.0) / 600.0).exp())
        } else if Value::ZERO < self {
            1.0
        } else {
            0.0
        }
    }
    pub fn mate_in(ply: i32) -> Value {
        Value::MATE - Value(ply)
    }
    pub fn mated_in(ply: i32) -> Value {
        -Value::MATE + Value(ply)
    }
    pub fn abs(self) -> Self {
        Value(self.0.abs() as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PieceType(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(pub i32);

impl PieceType {
    const PROMOTION: i32 = 8;
    pub const OCCUPIED: PieceType = PieceType(0);
    pub const PAWN: PieceType = PieceType(1);
    pub const LANCE: PieceType = PieceType(2);
    pub const KNIGHT: PieceType = PieceType(3);
    pub const SILVER: PieceType = PieceType(4);
    pub const BISHOP: PieceType = PieceType(5);
    pub const ROOK: PieceType = PieceType(6);
    pub const GOLD: PieceType = PieceType(7);
    pub const KING: PieceType = PieceType(8);
    pub const PRO_PAWN: PieceType = PieceType(9);
    pub const PRO_LANCE: PieceType = PieceType(10);
    pub const PRO_KNIGHT: PieceType = PieceType(11);
    pub const PRO_SILVER: PieceType = PieceType(12);
    pub const HORSE: PieceType = PieceType(13);
    pub const DRAGON: PieceType = PieceType(14);
    pub const NUM: usize = 15;
    pub const HAND_NUM: usize = 8;

    // The order of these elements is specified with SFEN.
    pub const ALL_HAND_FOR_SFEN: [PieceType; 7] = [
        PieceType::ROOK,
        PieceType::BISHOP,
        PieceType::GOLD,
        PieceType::SILVER,
        PieceType::KNIGHT,
        PieceType::LANCE,
        PieceType::PAWN,
    ];

    pub const ALL_HAND: [PieceType; 7] = [
        PieceType::PAWN,
        PieceType::LANCE,
        PieceType::KNIGHT,
        PieceType::SILVER,
        PieceType::BISHOP,
        PieceType::ROOK,
        PieceType::GOLD,
    ];

    pub fn new(pc: Piece) -> PieceType {
        PieceType(pc.0 & (Piece::WHITE_BIT - 1))
    }
    #[allow(dead_code)]
    fn is_slider(self) -> bool {
        const IS_SLIDER_VAL: u32 = (1 << PieceType::LANCE.0)
            | (1 << PieceType::BISHOP.0)
            | (1 << PieceType::ROOK.0)
            | (1 << PieceType::HORSE.0)
            | (1 << PieceType::DRAGON.0);
        (IS_SLIDER_VAL & (1 << self.0)) != 0
    }
    pub fn is_promotable(self) -> bool {
        matches!(
            self,
            PieceType::PAWN | PieceType::LANCE | PieceType::KNIGHT | PieceType::SILVER | PieceType::BISHOP | PieceType::ROOK
        )
    }
    pub fn to_promote(self) -> PieceType {
        debug_assert!(self.is_promotable());
        PieceType(self.0 + PieceType::PROMOTION)
    }
    pub fn to_demote_if_possible(self) -> PieceType {
        match self {
            PieceType::PAWN | PieceType::PRO_PAWN => PieceType::PAWN,
            PieceType::LANCE | PieceType::PRO_LANCE => PieceType::LANCE,
            PieceType::KNIGHT | PieceType::PRO_KNIGHT => PieceType::KNIGHT,
            PieceType::SILVER | PieceType::PRO_SILVER => PieceType::SILVER,
            PieceType::BISHOP | PieceType::HORSE => PieceType::BISHOP,
            PieceType::ROOK | PieceType::DRAGON => PieceType::ROOK,
            PieceType::GOLD => PieceType::GOLD,
            _ => unreachable!(),
        }
    }
    pub fn to_usi_str(self) -> &'static str {
        match self {
            PieceType::PAWN => "P",
            PieceType::LANCE => "L",
            PieceType::KNIGHT => "N",
            PieceType::SILVER => "S",
            PieceType::BISHOP => "B",
            PieceType::ROOK => "R",
            PieceType::GOLD => "G",
            PieceType::KING => "K",
            PieceType::PRO_PAWN => "+P",
            PieceType::PRO_LANCE => "+L",
            PieceType::PRO_KNIGHT => "+N",
            PieceType::PRO_SILVER => "+S",
            PieceType::HORSE => "+B",
            PieceType::DRAGON => "+R",
            _ => unreachable!(),
        }
    }
    pub fn to_csa_str(self) -> &'static str {
        match self {
            PieceType::PAWN => "FU",
            PieceType::LANCE => "KY",
            PieceType::KNIGHT => "KE",
            PieceType::SILVER => "GI",
            PieceType::BISHOP => "KA",
            PieceType::ROOK => "HI",
            PieceType::GOLD => "KI",
            PieceType::KING => "OU",
            PieceType::PRO_PAWN => "TO",
            PieceType::PRO_LANCE => "NY",
            PieceType::PRO_KNIGHT => "NK",
            PieceType::PRO_SILVER => "NG",
            PieceType::HORSE => "UM",
            PieceType::DRAGON => "RY",
            _ => unreachable!(),
        }
    }
    pub fn new_from_str_for_drop_move(s: &str) -> Option<PieceType> {
        match s {
            "P" => Some(PieceType::PAWN),
            "L" => Some(PieceType::LANCE),
            "N" => Some(PieceType::KNIGHT),
            "S" => Some(PieceType::SILVER),
            "B" => Some(PieceType::BISHOP),
            "R" => Some(PieceType::ROOK),
            "G" => Some(PieceType::GOLD),
            "K" => Some(PieceType::KING),
            _ => None,
        }
    }
    pub fn new_from_csa_str(s: &str) -> Option<PieceType> {
        match s {
            "FU" => Some(PieceType::PAWN),
            "KY" => Some(PieceType::LANCE),
            "KE" => Some(PieceType::KNIGHT),
            "GI" => Some(PieceType::SILVER),
            "KA" => Some(PieceType::BISHOP),
            "HI" => Some(PieceType::ROOK),
            "KI" => Some(PieceType::GOLD),
            "OU" => Some(PieceType::KING),
            "TO" => Some(PieceType::PRO_PAWN),
            "NY" => Some(PieceType::PRO_LANCE),
            "NK" => Some(PieceType::PRO_KNIGHT),
            "NG" => Some(PieceType::PRO_SILVER),
            "UM" => Some(PieceType::HORSE),
            "RY" => Some(PieceType::DRAGON),
            _ => None,
        }
    }
}

impl Piece {
    pub const PROMOTION: i32 = 8;
    pub const WHITE_BIT_SHIFT: i32 = 4;
    pub const WHITE_BIT: i32 = 1 << Piece::WHITE_BIT_SHIFT;
    pub const EMPTY: Piece = Piece(0);
    pub const B_PAWN: Piece = Piece(1);
    pub const B_LANCE: Piece = Piece(2);
    pub const B_KNIGHT: Piece = Piece(3);
    pub const B_SILVER: Piece = Piece(4);
    pub const B_BISHOP: Piece = Piece(5);
    pub const B_ROOK: Piece = Piece(6);
    pub const B_GOLD: Piece = Piece(7);
    pub const B_KING: Piece = Piece(8);
    pub const B_PRO_PAWN: Piece = Piece(9);
    pub const B_PRO_LANCE: Piece = Piece(10);
    pub const B_PRO_KNIGHT: Piece = Piece(11);
    pub const B_PRO_SILVER: Piece = Piece(12);
    pub const B_HORSE: Piece = Piece(13);
    pub const B_DRAGON: Piece = Piece(14);
    pub const W_PAWN: Piece = Piece(17);
    pub const W_LANCE: Piece = Piece(18);
    pub const W_KNIGHT: Piece = Piece(19);
    pub const W_SILVER: Piece = Piece(20);
    pub const W_BISHOP: Piece = Piece(21);
    pub const W_ROOK: Piece = Piece(22);
    pub const W_GOLD: Piece = Piece(23);
    pub const W_KING: Piece = Piece(24);
    pub const W_PRO_PAWN: Piece = Piece(25);
    pub const W_PRO_LANCE: Piece = Piece(26);
    pub const W_PRO_KNIGHT: Piece = Piece(27);
    pub const W_PRO_SILVER: Piece = Piece(28);
    pub const W_HORSE: Piece = Piece(29);
    pub const W_DRAGON: Piece = Piece(30);

    pub const NUM: usize = Piece::W_DRAGON.0 as usize + 1;
    pub fn new(c: Color, pt: PieceType) -> Piece {
        Piece((c.0 << 4) | pt.0)
    }
    pub fn new_from_str(s: &str) -> Option<Piece> {
        match s {
            "P" => Some(Piece::B_PAWN),
            "p" => Some(Piece::W_PAWN),
            "L" => Some(Piece::B_LANCE),
            "l" => Some(Piece::W_LANCE),
            "N" => Some(Piece::B_KNIGHT),
            "n" => Some(Piece::W_KNIGHT),
            "S" => Some(Piece::B_SILVER),
            "s" => Some(Piece::W_SILVER),
            "B" => Some(Piece::B_BISHOP),
            "b" => Some(Piece::W_BISHOP),
            "R" => Some(Piece::B_ROOK),
            "r" => Some(Piece::W_ROOK),
            "G" => Some(Piece::B_GOLD),
            "g" => Some(Piece::W_GOLD),
            "K" => Some(Piece::B_KING),
            "k" => Some(Piece::W_KING),
            "+P" => Some(Piece::B_PRO_PAWN),
            "+p" => Some(Piece::W_PRO_PAWN),
            "+L" => Some(Piece::B_PRO_LANCE),
            "+l" => Some(Piece::W_PRO_LANCE),
            "+N" => Some(Piece::B_PRO_KNIGHT),
            "+n" => Some(Piece::W_PRO_KNIGHT),
            "+S" => Some(Piece::B_PRO_SILVER),
            "+s" => Some(Piece::W_PRO_SILVER),
            "+B" => Some(Piece::B_HORSE),
            "+b" => Some(Piece::W_HORSE),
            "+R" => Some(Piece::B_DRAGON),
            "+r" => Some(Piece::W_DRAGON),
            _ => None,
        }
    }
    pub fn new_hand_piece_from_str(s: &str) -> Option<Piece> {
        match s {
            "P" => Some(Piece::B_PAWN),
            "p" => Some(Piece::W_PAWN),
            "L" => Some(Piece::B_LANCE),
            "l" => Some(Piece::W_LANCE),
            "N" => Some(Piece::B_KNIGHT),
            "n" => Some(Piece::W_KNIGHT),
            "S" => Some(Piece::B_SILVER),
            "s" => Some(Piece::W_SILVER),
            "B" => Some(Piece::B_BISHOP),
            "b" => Some(Piece::W_BISHOP),
            "R" => Some(Piece::B_ROOK),
            "r" => Some(Piece::W_ROOK),
            "G" => Some(Piece::B_GOLD),
            "g" => Some(Piece::W_GOLD),
            "K" => Some(Piece::B_KING),
            "k" => Some(Piece::W_KING),
            _ => None,
        }
    }
    pub fn inverse(self) -> Piece {
        let pt = PieceType::new(self);
        let c = Color::new(self);
        Piece::new(c.inverse(), pt)
    }
    pub fn is_promotable(self) -> bool {
        matches!(
            self,
            Piece::B_PAWN
                | Piece::B_LANCE
                | Piece::B_KNIGHT
                | Piece::B_SILVER
                | Piece::B_BISHOP
                | Piece::B_ROOK
                | Piece::W_PAWN
                | Piece::W_LANCE
                | Piece::W_KNIGHT
                | Piece::W_SILVER
                | Piece::W_BISHOP
                | Piece::W_ROOK
        )
    }
    pub fn to_promote(self) -> Piece {
        debug_assert!(self.is_promotable());
        Piece(self.0 + Piece::PROMOTION)
    }
    pub fn to_demote(self) -> Piece {
        debug_assert!(!self.is_promotable());
        Piece(self.0 - Piece::PROMOTION)
    }
    pub fn is_king(self) -> bool {
        PieceType::new(self) == PieceType::KING
    }
    pub fn to_usi_str(self) -> &'static str {
        match self {
            Piece::EMPTY => "",
            Piece::B_PAWN => "P",
            Piece::B_LANCE => "L",
            Piece::B_KNIGHT => "N",
            Piece::B_SILVER => "S",
            Piece::B_BISHOP => "B",
            Piece::B_ROOK => "R",
            Piece::B_GOLD => "G",
            Piece::B_KING => "K",
            Piece::B_PRO_PAWN => "+P",
            Piece::B_PRO_LANCE => "+L",
            Piece::B_PRO_KNIGHT => "+N",
            Piece::B_PRO_SILVER => "+S",
            Piece::B_HORSE => "+B",
            Piece::B_DRAGON => "+R",
            Piece::W_PAWN => "p",
            Piece::W_LANCE => "l",
            Piece::W_KNIGHT => "n",
            Piece::W_SILVER => "s",
            Piece::W_BISHOP => "b",
            Piece::W_ROOK => "r",
            Piece::W_GOLD => "g",
            Piece::W_KING => "k",
            Piece::W_PRO_PAWN => "+p",
            Piece::W_PRO_LANCE => "+l",
            Piece::W_PRO_KNIGHT => "+n",
            Piece::W_PRO_SILVER => "+s",
            Piece::W_HORSE => "+b",
            Piece::W_DRAGON => "+r",
            _ => unreachable!(),
        }
    }
    pub fn to_csa_str(self) -> &'static str {
        match self {
            Piece::EMPTY => " * ",
            Piece::B_PAWN => "+FU",
            Piece::B_LANCE => "+KY",
            Piece::B_KNIGHT => "+KE",
            Piece::B_SILVER => "+GI",
            Piece::B_BISHOP => "+KA",
            Piece::B_ROOK => "+HI",
            Piece::B_GOLD => "+KI",
            Piece::B_KING => "+OU",
            Piece::B_PRO_PAWN => "+TO",
            Piece::B_PRO_LANCE => "+NY",
            Piece::B_PRO_KNIGHT => "+NK",
            Piece::B_PRO_SILVER => "+NG",
            Piece::B_HORSE => "+UM",
            Piece::B_DRAGON => "+RY",
            Piece::W_PAWN => "-FU",
            Piece::W_LANCE => "-KY",
            Piece::W_KNIGHT => "-KE",
            Piece::W_SILVER => "-GI",
            Piece::W_BISHOP => "-KA",
            Piece::W_ROOK => "-HI",
            Piece::W_GOLD => "-KI",
            Piece::W_KING => "-OU",
            Piece::W_PRO_PAWN => "-TO",
            Piece::W_PRO_LANCE => "-NY",
            Piece::W_PRO_KNIGHT => "-NK",
            Piece::W_PRO_SILVER => "-NG",
            Piece::W_HORSE => "-UM",
            Piece::W_DRAGON => "-RY",
            _ => unreachable!(),
        }
    }
}

pub struct PawnType;
pub struct LanceType;
pub struct KnightType;
pub struct SilverType;
pub struct BishopType;
pub struct RookType;
pub struct GoldType;
pub struct KingType;
#[allow(dead_code)]
pub struct ProPawnType;
#[allow(dead_code)]
pub struct ProLanceType;
#[allow(dead_code)]
pub struct ProKnightType;
#[allow(dead_code)]
pub struct ProSilverType;
pub struct HorseType;
pub struct DragonType;

pub trait PieceTypeTrait {
    const PIECE_TYPE: PieceType;
}
impl PieceTypeTrait for PawnType {
    const PIECE_TYPE: PieceType = PieceType::PAWN;
}
impl PieceTypeTrait for LanceType {
    const PIECE_TYPE: PieceType = PieceType::LANCE;
}
impl PieceTypeTrait for KnightType {
    const PIECE_TYPE: PieceType = PieceType::KNIGHT;
}
impl PieceTypeTrait for SilverType {
    const PIECE_TYPE: PieceType = PieceType::SILVER;
}
impl PieceTypeTrait for BishopType {
    const PIECE_TYPE: PieceType = PieceType::BISHOP;
}
impl PieceTypeTrait for RookType {
    const PIECE_TYPE: PieceType = PieceType::ROOK;
}
impl PieceTypeTrait for GoldType {
    const PIECE_TYPE: PieceType = PieceType::GOLD;
}
impl PieceTypeTrait for KingType {
    const PIECE_TYPE: PieceType = PieceType::KING;
}
impl PieceTypeTrait for ProPawnType {
    const PIECE_TYPE: PieceType = PieceType::PRO_PAWN;
}
impl PieceTypeTrait for ProLanceType {
    const PIECE_TYPE: PieceType = PieceType::PRO_LANCE;
}
impl PieceTypeTrait for ProKnightType {
    const PIECE_TYPE: PieceType = PieceType::PRO_KNIGHT;
}
impl PieceTypeTrait for ProSilverType {
    const PIECE_TYPE: PieceType = PieceType::PRO_SILVER;
}
impl PieceTypeTrait for HorseType {
    const PIECE_TYPE: PieceType = PieceType::HORSE;
}
impl PieceTypeTrait for DragonType {
    const PIECE_TYPE: PieceType = PieceType::DRAGON;
}

pub const MAX_PLY: i32 = 246;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign, Sub, SubAssign, Mul, Div)]
pub struct Depth(pub i32);

impl Depth {
    pub const ONE_PLY: Depth = Depth(1);
    pub const ZERO: Depth = Depth(0);
    pub const QS_CHECKS: Depth = Depth(0);
    pub const QS_NO_CHECKS: Depth = Depth(-1);
    pub const QS_RECAPTURES: Depth = Depth(-5);
    pub const NONE: Depth = Depth(-6);
    pub const OFFSET: Depth = Depth(-7);
    pub const MAX: Depth = Depth(MAX_PLY);
}

#[derive(Clone, Copy, PartialEq, Eq, BitXor, BitXorAssign, Hash)]
pub struct Key(pub u64);
#[derive(Clone, Copy, PartialEq, Eq, BitXor, BitXorAssign, Hash)]
pub struct KeyExcludedTurn(pub u64);

impl Key {
    #[inline]
    pub fn make_key(seed: u64) -> Key {
        Key(seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) & !1)
    }
    #[inline]
    pub fn excluded_turn(self) -> KeyExcludedTurn {
        KeyExcludedTurn(self.0 >> 1)
    }
    #[allow(dead_code)]
    #[inline]
    pub fn turn_bit(self) -> u64 {
        self.0 & 1
    }
}

#[test]
fn test_color_inverse() {
    assert_eq!(Color::BLACK.inverse(), Color::WHITE);
    assert_eq!(Color::WHITE.inverse(), Color::BLACK);
}

#[test]
fn test_square_new() {
    assert_eq!(Square::new(File::FILE3, Rank::RANK4), Square::SQ34);
}

#[test]
fn test_square_inverse() {
    assert_eq!(Square::SQ11.inverse(), Square::SQ99);
    assert_eq!(Square::SQ12.inverse(), Square::SQ98);
    assert_eq!(Square::SQ21.inverse(), Square::SQ89);
    assert_eq!(Square::SQ99.inverse(), Square::SQ11);
    for i in 0..(Square::NUM as i32) {
        let sq = Square(i);
        assert_eq!(sq.inverse().inverse(), sq);
    }
}

#[test]
fn test_square_inverse_file() {
    assert_eq!(Square::SQ11.inverse_file(), Square::SQ91);
    assert_eq!(Square::SQ12.inverse_file(), Square::SQ92);
    assert_eq!(Square::SQ21.inverse_file(), Square::SQ81);
    assert_eq!(Square::SQ99.inverse_file(), Square::SQ19);
    for i in 0..(Square::NUM as i32) {
        let sq = Square(i);
        assert_eq!(sq.inverse_file().inverse_file(), sq);
    }
}

#[test]
fn test_file_new_and_rank_new() {
    for i in 0..(Square::NUM as i32) {
        let sq = Square(i);
        let file = File::new(sq);
        let rank = Rank::new(sq);
        assert_eq!(Square::new(file, rank), sq);
    }
}

#[test]
fn test_file_inverse() {
    assert_eq!(File::FILE1.inverse(), File::FILE9);
    assert_eq!(File::FILE2.inverse(), File::FILE8);
    for i in 0..(File::NUM as i32) {
        let f = File(i);
        assert_eq!(f.inverse().inverse(), f);
    }
}

#[test]
fn test_rank_inverse() {
    assert_eq!(Rank::RANK1.inverse(), Rank::RANK9);
    assert_eq!(Rank::RANK2.inverse(), Rank::RANK8);
    for i in 0..(Rank::NUM as i32) {
        let r = Rank(i);
        assert_eq!(r.inverse().inverse(), r);
    }
}

#[test]
fn test_file_new() {
    for sq in Square::ALL.iter() {
        use crate::types::Square as S;
        #[rustfmt::skip]
        let ans = match *sq {
            S::SQ11 | S::SQ12 | S::SQ13 | S::SQ14 | S::SQ15 | S::SQ16 | S::SQ17 | S::SQ18 | S::SQ19 => File::FILE1,
            S::SQ21 | S::SQ22 | S::SQ23 | S::SQ24 | S::SQ25 | S::SQ26 | S::SQ27 | S::SQ28 | S::SQ29 => File::FILE2,
            S::SQ31 | S::SQ32 | S::SQ33 | S::SQ34 | S::SQ35 | S::SQ36 | S::SQ37 | S::SQ38 | S::SQ39 => File::FILE3,
            S::SQ41 | S::SQ42 | S::SQ43 | S::SQ44 | S::SQ45 | S::SQ46 | S::SQ47 | S::SQ48 | S::SQ49 => File::FILE4,
            S::SQ51 | S::SQ52 | S::SQ53 | S::SQ54 | S::SQ55 | S::SQ56 | S::SQ57 | S::SQ58 | S::SQ59 => File::FILE5,
            S::SQ61 | S::SQ62 | S::SQ63 | S::SQ64 | S::SQ65 | S::SQ66 | S::SQ67 | S::SQ68 | S::SQ69 => File::FILE6,
            S::SQ71 | S::SQ72 | S::SQ73 | S::SQ74 | S::SQ75 | S::SQ76 | S::SQ77 | S::SQ78 | S::SQ79 => File::FILE7,
            S::SQ81 | S::SQ82 | S::SQ83 | S::SQ84 | S::SQ85 | S::SQ86 | S::SQ87 | S::SQ88 | S::SQ89 => File::FILE8,
            S::SQ91 | S::SQ92 | S::SQ93 | S::SQ94 | S::SQ95 | S::SQ96 | S::SQ97 | S::SQ98 | S::SQ99 => File::FILE9,
            _ => unreachable!(),
        };
        assert_eq!(File::new(*sq), ans);
    }
}

#[test]
fn test_rank_new() {
    for sq in Square::ALL.iter() {
        use crate::types::Square as S;
        #[rustfmt::skip]
        let ans = match *sq {
            S::SQ11 | S::SQ21 | S::SQ31 | S::SQ41 | S::SQ51 | S::SQ61 | S::SQ71 | S::SQ81 | S::SQ91 => Rank::RANK1,
            S::SQ12 | S::SQ22 | S::SQ32 | S::SQ42 | S::SQ52 | S::SQ62 | S::SQ72 | S::SQ82 | S::SQ92 => Rank::RANK2,
            S::SQ13 | S::SQ23 | S::SQ33 | S::SQ43 | S::SQ53 | S::SQ63 | S::SQ73 | S::SQ83 | S::SQ93 => Rank::RANK3,
            S::SQ14 | S::SQ24 | S::SQ34 | S::SQ44 | S::SQ54 | S::SQ64 | S::SQ74 | S::SQ84 | S::SQ94 => Rank::RANK4,
            S::SQ15 | S::SQ25 | S::SQ35 | S::SQ45 | S::SQ55 | S::SQ65 | S::SQ75 | S::SQ85 | S::SQ95 => Rank::RANK5,
            S::SQ16 | S::SQ26 | S::SQ36 | S::SQ46 | S::SQ56 | S::SQ66 | S::SQ76 | S::SQ86 | S::SQ96 => Rank::RANK6,
            S::SQ17 | S::SQ27 | S::SQ37 | S::SQ47 | S::SQ57 | S::SQ67 | S::SQ77 | S::SQ87 | S::SQ97 => Rank::RANK7,
            S::SQ18 | S::SQ28 | S::SQ38 | S::SQ48 | S::SQ58 | S::SQ68 | S::SQ78 | S::SQ88 | S::SQ98 => Rank::RANK8,
            S::SQ19 | S::SQ29 | S::SQ39 | S::SQ49 | S::SQ59 | S::SQ69 | S::SQ79 | S::SQ89 | S::SQ99 => Rank::RANK9,
            _ => unreachable!(),
        };
        assert_eq!(Rank::new(*sq), ans);
    }
}

#[test]
fn test_piece_type_new() {
    assert_eq!(PieceType::PAWN, PieceType::new(Piece::B_PAWN));
    assert_eq!(PieceType::PAWN, PieceType::new(Piece::W_PAWN));
    assert_eq!(PieceType::LANCE, PieceType::new(Piece::B_LANCE));
    assert_eq!(PieceType::LANCE, PieceType::new(Piece::W_LANCE));
    assert_eq!(PieceType::DRAGON, PieceType::new(Piece::B_DRAGON));
    assert_eq!(PieceType::DRAGON, PieceType::new(Piece::W_DRAGON));
}

#[test]
fn test_piece_new() {
    assert_eq!(Piece::B_KING, Piece::new(Color::BLACK, PieceType::KING));
    assert_eq!(Piece::W_KING, Piece::new(Color::WHITE, PieceType::KING));
    assert_eq!(Piece::B_DRAGON, Piece::new(Color::BLACK, PieceType::DRAGON));
    assert_eq!(Piece::W_DRAGON, Piece::new(Color::WHITE, PieceType::DRAGON));
}

#[test]
fn test_piece_inverse() {
    assert_eq!(Piece::B_PAWN.inverse(), Piece::W_PAWN);
    assert_eq!(Piece::B_LANCE.inverse(), Piece::W_LANCE);
    assert_eq!(Piece::B_KNIGHT.inverse(), Piece::W_KNIGHT);
    assert_eq!(Piece::B_SILVER.inverse(), Piece::W_SILVER);
    assert_eq!(Piece::B_BISHOP.inverse(), Piece::W_BISHOP);
    assert_eq!(Piece::B_ROOK.inverse(), Piece::W_ROOK);
    assert_eq!(Piece::B_GOLD.inverse(), Piece::W_GOLD);
    assert_eq!(Piece::B_KING.inverse(), Piece::W_KING);
    assert_eq!(Piece::B_PRO_PAWN.inverse(), Piece::W_PRO_PAWN);
    assert_eq!(Piece::B_PRO_LANCE.inverse(), Piece::W_PRO_LANCE);
    assert_eq!(Piece::B_PRO_KNIGHT.inverse(), Piece::W_PRO_KNIGHT);
    assert_eq!(Piece::B_PRO_SILVER.inverse(), Piece::W_PRO_SILVER);
    assert_eq!(Piece::B_HORSE.inverse(), Piece::W_HORSE);
    assert_eq!(Piece::B_DRAGON.inverse(), Piece::W_DRAGON);
}

#[test]
fn test_is_slider() {
    assert!(!PieceType::OCCUPIED.is_slider());
    assert!(!PieceType::PAWN.is_slider());
    assert!(PieceType::LANCE.is_slider());
    assert!(!PieceType::KNIGHT.is_slider());
    assert!(!PieceType::SILVER.is_slider());
    assert!(!PieceType::GOLD.is_slider());
    assert!(PieceType::BISHOP.is_slider());
    assert!(PieceType::ROOK.is_slider());
    assert!(!PieceType::KING.is_slider());
    assert!(!PieceType::PRO_PAWN.is_slider());
    assert!(!PieceType::PRO_LANCE.is_slider());
    assert!(!PieceType::PRO_KNIGHT.is_slider());
    assert!(!PieceType::PRO_SILVER.is_slider());
    assert!(PieceType::HORSE.is_slider());
    assert!(PieceType::DRAGON.is_slider());
}

#[test]
fn test_relation_new() {
    assert_eq!(Relation::new(Square::SQ11, Square::SQ15), Relation::FILE_NS);
    assert_eq!(Relation::new(Square::SQ15, Square::SQ11), Relation::FILE_SN);
    assert_eq!(Relation::new(Square::SQ11, Square::SQ71), Relation::RANK_EW);
    assert_eq!(Relation::new(Square::SQ71, Square::SQ11), Relation::RANK_WE);
    assert_eq!(Relation::new(Square::SQ11, Square::SQ33), Relation::DIAG_NESW);
    assert_eq!(Relation::new(Square::SQ33, Square::SQ11), Relation::DIAG_SWNE);
    assert_eq!(Relation::new(Square::SQ11, Square::SQ23), Relation::MISC);
    assert_eq!(Relation::new(Square::SQ91, Square::SQ19), Relation::DIAG_NWSE);
    assert_eq!(Relation::new(Square::SQ19, Square::SQ91), Relation::DIAG_SENW);
    for sq0 in Square::ALL.iter() {
        let f1 = File::new(*sq0);
        for r1 in Rank::ALL.iter() {
            let sq1 = Square::new(f1, *r1);
            if *sq0 == sq1 {
                assert_eq!(Relation::new(*sq0, sq1), Relation::MISC);
            } else {
                assert!(Relation::new(*sq0, sq1).is_file());
            }
        }
    }
    for sq0 in Square::ALL.iter() {
        let r1 = Rank::new(*sq0);
        for f1 in File::ALL.iter() {
            let sq1 = Square::new(*f1, r1);
            if *sq0 == sq1 {
                assert_eq!(Relation::new(*sq0, sq1), Relation::MISC);
            } else {
                assert!(Relation::new(*sq0, sq1).is_rank());
            }
        }
    }
}

#[test]
fn test_square_to_usi_string() {
    assert_eq!(Square::SQ11.to_usi_string(), "1a");
    assert_eq!(Square::SQ99.to_usi_string(), "9i");
    assert_eq!(Square::SQ35.to_usi_string(), "3e");
}

#[test]
fn test_square_to_csa_string() {
    assert_eq!(Square::SQ11.to_csa_string(), "11");
    assert_eq!(Square::SQ99.to_csa_string(), "99");
    assert_eq!(Square::SQ35.to_csa_string(), "35");
}

#[test]
fn test_is_opponent_field() {
    assert!(Rank::RANK1.is_opponent_field(Color::BLACK));
    assert!(Rank::RANK2.is_opponent_field(Color::BLACK));
    assert!(Rank::RANK3.is_opponent_field(Color::BLACK));
    assert!(!Rank::RANK4.is_opponent_field(Color::BLACK));
    assert!(!Rank::RANK5.is_opponent_field(Color::BLACK));
    assert!(!Rank::RANK6.is_opponent_field(Color::BLACK));
    assert!(!Rank::RANK7.is_opponent_field(Color::BLACK));
    assert!(!Rank::RANK8.is_opponent_field(Color::BLACK));
    assert!(!Rank::RANK9.is_opponent_field(Color::BLACK));
    assert!(!Rank::RANK1.is_opponent_field(Color::WHITE));
    assert!(!Rank::RANK2.is_opponent_field(Color::WHITE));
    assert!(!Rank::RANK3.is_opponent_field(Color::WHITE));
    assert!(!Rank::RANK4.is_opponent_field(Color::WHITE));
    assert!(!Rank::RANK5.is_opponent_field(Color::WHITE));
    assert!(!Rank::RANK6.is_opponent_field(Color::WHITE));
    assert!(Rank::RANK7.is_opponent_field(Color::WHITE));
    assert!(Rank::RANK8.is_opponent_field(Color::WHITE));
    assert!(Rank::RANK9.is_opponent_field(Color::WHITE));
}

#[test]
fn test_is_in_front_of() {
    assert_eq!(
        Rank::RANK1,
        Rank::new_from_color_and_rank_as_black(Color::BLACK, RankAsBlack::RANK1)
    );
    assert_eq!(
        Rank::RANK9,
        Rank::new_from_color_and_rank_as_black(Color::WHITE, RankAsBlack::RANK1)
    );

    assert!(!Rank::RANK1.is_in_front_of(Color::BLACK, RankAsBlack::RANK1));
    assert!(Rank::RANK1.is_in_front_of(Color::BLACK, RankAsBlack::RANK2));

    assert!(!Rank::RANK9.is_in_front_of(Color::WHITE, RankAsBlack::RANK1));
    assert!(Rank::RANK9.is_in_front_of(Color::WHITE, RankAsBlack::RANK2));
}

#[test]
fn test_is_aligned_and_sq2_is_not_between_sq0_and_sq1() {
    assert!(is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ11,
        Square::SQ12,
        Square::SQ13
    ));
    assert!(!is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ11,
        Square::SQ13,
        Square::SQ12
    ));
    assert!(is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ11,
        Square::SQ22,
        Square::SQ33
    ));
    assert!(is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ22,
        Square::SQ11,
        Square::SQ33
    ));
    assert!(!is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ33,
        Square::SQ11,
        Square::SQ22
    ));
    assert!(is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ99,
        Square::SQ88,
        Square::SQ77
    ));
    assert!(is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ99,
        Square::SQ98,
        Square::SQ97
    ));
    assert!(is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ91,
        Square::SQ82,
        Square::SQ73
    ));
    assert!(is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ73,
        Square::SQ82,
        Square::SQ91
    ));
    assert!(is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ91,
        Square::SQ81,
        Square::SQ71
    ));
    assert!(is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ71,
        Square::SQ81,
        Square::SQ91
    ));
    assert!(!is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ71,
        Square::SQ91,
        Square::SQ81
    ));
    assert!(!is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ11,
        Square::SQ21,
        Square::SQ42
    ));
    assert!(!is_aligned_and_sq2_is_not_between_sq0_and_sq1(
        Square::SQ11,
        Square::SQ32,
        Square::SQ53
    ));
}

#[test]
fn test_bound() {
    assert!(!Bound::BOUND_NONE.include_lower());
    assert!(!Bound::BOUND_NONE.include_upper());
    assert!(Bound::LOWER.include_lower());
    assert!(!Bound::LOWER.include_upper());
    assert!(!Bound::UPPER.include_lower());
    assert!(Bound::UPPER.include_upper());
    assert!(Bound::EXACT.include_lower());
    assert!(Bound::EXACT.include_upper());
}
