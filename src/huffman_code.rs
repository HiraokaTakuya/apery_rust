use crate::position::*;
use crate::types::*;
use thiserror::Error;

#[derive(PartialEq, Eq)]
pub struct HuffmanCode {
    pub value: u8,
    pub bit_length: u8,
}

#[rustfmt::skip]
impl HuffmanCode {
    const EMPTY: HuffmanCode = HuffmanCode{value: 0b0, bit_length: 1};
    const B_PAWN: HuffmanCode = HuffmanCode{value: 0b1, bit_length: 4};
    const B_LANCE: HuffmanCode = HuffmanCode{value: 0b11, bit_length: 6};
    const B_KNIGHT: HuffmanCode = HuffmanCode{value: 0b111, bit_length: 6};
    const B_SILVER: HuffmanCode = HuffmanCode{value: 0b1011, bit_length: 6};
    const B_BISHOP: HuffmanCode = HuffmanCode{value: 0b1_1111, bit_length: 8};
    const B_ROOK: HuffmanCode = HuffmanCode{value: 0b11_1111, bit_length: 8};
    const B_GOLD: HuffmanCode = HuffmanCode{value: 0b1111, bit_length: 6};
    const B_PRO_PAWN: HuffmanCode = HuffmanCode{value: 0b1001, bit_length: 4};
    const B_PRO_LANCE: HuffmanCode = HuffmanCode{value: 0b10_0011, bit_length: 6};
    const B_PRO_KNIGHT: HuffmanCode = HuffmanCode{value: 0b10_0111, bit_length: 6};
    const B_PRO_SILVER: HuffmanCode = HuffmanCode{value: 0b10_1011, bit_length: 6};
    const B_HORSE: HuffmanCode = HuffmanCode{value: 0b1001_1111, bit_length: 8};
    const B_DRAGON: HuffmanCode = HuffmanCode{value: 0b1011_1111, bit_length: 8};
    const W_PAWN: HuffmanCode = HuffmanCode{value: 0b101, bit_length: 4};
    const W_LANCE: HuffmanCode = HuffmanCode{value: 0b1_0011, bit_length: 6};
    const W_KNIGHT: HuffmanCode = HuffmanCode{value: 0b1_0111, bit_length: 6};
    const W_SILVER: HuffmanCode = HuffmanCode{value: 0b1_1011, bit_length: 6};
    const W_BISHOP: HuffmanCode = HuffmanCode{value: 0b101_1111, bit_length: 8};
    const W_ROOK: HuffmanCode = HuffmanCode{value: 0b111_1111, bit_length: 8};
    const W_GOLD: HuffmanCode = HuffmanCode{value: 0b10_1111, bit_length: 6};
    const W_PRO_PAWN: HuffmanCode = HuffmanCode{value: 0b1101, bit_length: 4};
    const W_PRO_LANCE: HuffmanCode = HuffmanCode{value: 0b11_0011, bit_length: 6};
    const W_PRO_KNIGHT: HuffmanCode = HuffmanCode{value: 0b11_0111, bit_length: 6};
    const W_PRO_SILVER: HuffmanCode = HuffmanCode{value: 0b11_1011, bit_length: 6};
    const W_HORSE: HuffmanCode = HuffmanCode{value: 0b1101_1111, bit_length: 8};
    const W_DRAGON: HuffmanCode = HuffmanCode{value: 0b1111_1111, bit_length: 8};
    const MAX_BIT_LENGTH_FOR_FIELD: u8 = 8;

    const B_HAND_PAWN: HuffmanCode = HuffmanCode{value: 0b0, bit_length: 3};
    const W_HAND_PAWN: HuffmanCode = HuffmanCode{value: 0b100, bit_length: 3};
    const B_HAND_LANCE: HuffmanCode = HuffmanCode{value: 0b1, bit_length: 5};
    const W_HAND_LANCE: HuffmanCode = HuffmanCode{value: 0b1_0001, bit_length: 5};
    const B_HAND_KNIGHT: HuffmanCode = HuffmanCode{value: 0b11, bit_length: 5};
    const W_HAND_KNIGHT: HuffmanCode = HuffmanCode{value: 0b1_0011, bit_length: 5};
    const B_HAND_SILVER: HuffmanCode = HuffmanCode{value: 0b101, bit_length: 5};
    const W_HAND_SILVER: HuffmanCode = HuffmanCode{value: 0b1_0101, bit_length: 5};
    const B_HAND_GOLD: HuffmanCode = HuffmanCode{value: 0b111, bit_length: 5};
    const W_HAND_GOLD: HuffmanCode = HuffmanCode{value: 0b1_0111, bit_length: 5};
    const B_HAND_BISHOP: HuffmanCode = HuffmanCode{value: 0b1_1111, bit_length: 7};
    const W_HAND_BISHOP: HuffmanCode = HuffmanCode{value: 0b101_1111, bit_length: 7};
    const B_HAND_ROOK: HuffmanCode = HuffmanCode{value: 0b11_1111, bit_length: 7};
    const W_HAND_ROOK: HuffmanCode = HuffmanCode{value: 0b111_1111, bit_length: 7};
    const MAX_BIT_LENGTH_FOR_HAND: u8 = 7;

    pub fn new(pc: Piece) -> HuffmanCode {
        match pc {
            Piece::EMPTY => HuffmanCode::EMPTY,
            Piece::B_PAWN => HuffmanCode::B_PAWN,
            Piece::B_LANCE => HuffmanCode::B_LANCE,
            Piece::B_KNIGHT => HuffmanCode::B_KNIGHT,
            Piece::B_SILVER => HuffmanCode::B_SILVER,
            Piece::B_BISHOP => HuffmanCode::B_BISHOP,
            Piece::B_ROOK => HuffmanCode::B_ROOK,
            Piece::B_GOLD => HuffmanCode::B_GOLD,
            Piece::B_PRO_PAWN => HuffmanCode::B_PRO_PAWN,
            Piece::B_PRO_LANCE => HuffmanCode::B_PRO_LANCE,
            Piece::B_PRO_KNIGHT => HuffmanCode::B_PRO_KNIGHT,
            Piece::B_PRO_SILVER => HuffmanCode::B_PRO_SILVER,
            Piece::B_HORSE => HuffmanCode::B_HORSE,
            Piece::B_DRAGON => HuffmanCode::B_DRAGON,
            Piece::W_PAWN => HuffmanCode::W_PAWN,
            Piece::W_LANCE => HuffmanCode::W_LANCE,
            Piece::W_KNIGHT => HuffmanCode::W_KNIGHT,
            Piece::W_SILVER => HuffmanCode::W_SILVER,
            Piece::W_BISHOP => HuffmanCode::W_BISHOP,
            Piece::W_ROOK => HuffmanCode::W_ROOK,
            Piece::W_GOLD => HuffmanCode::W_GOLD,
            Piece::W_PRO_PAWN => HuffmanCode::W_PRO_PAWN,
            Piece::W_PRO_LANCE => HuffmanCode::W_PRO_LANCE,
            Piece::W_PRO_KNIGHT => HuffmanCode::W_PRO_KNIGHT,
            Piece::W_PRO_SILVER => HuffmanCode::W_PRO_SILVER,
            Piece::W_HORSE => HuffmanCode::W_HORSE,
            Piece::W_DRAGON => HuffmanCode::W_DRAGON,
            _ => unreachable!(),
        }
    }
    pub fn new_from_color_and_hand_piece_type(c: Color, pt: PieceType) -> HuffmanCode {
        match (c, pt) {
            (Color::BLACK, PieceType::PAWN) => HuffmanCode::B_HAND_PAWN,
            (Color::BLACK, PieceType::LANCE) => HuffmanCode::B_HAND_LANCE,
            (Color::BLACK, PieceType::KNIGHT) => HuffmanCode::B_HAND_KNIGHT,
            (Color::BLACK, PieceType::SILVER) => HuffmanCode::B_HAND_SILVER,
            (Color::BLACK, PieceType::BISHOP) => HuffmanCode::B_HAND_BISHOP,
            (Color::BLACK, PieceType::ROOK) => HuffmanCode::B_HAND_ROOK,
            (Color::BLACK, PieceType::GOLD) => HuffmanCode::B_HAND_GOLD,
            (Color::WHITE, PieceType::PAWN) => HuffmanCode::W_HAND_PAWN,
            (Color::WHITE, PieceType::LANCE) => HuffmanCode::W_HAND_LANCE,
            (Color::WHITE, PieceType::KNIGHT) => HuffmanCode::W_HAND_KNIGHT,
            (Color::WHITE, PieceType::SILVER) => HuffmanCode::W_HAND_SILVER,
            (Color::WHITE, PieceType::BISHOP) => HuffmanCode::W_HAND_BISHOP,
            (Color::WHITE, PieceType::ROOK) => HuffmanCode::W_HAND_ROOK,
            (Color::WHITE, PieceType::GOLD) => HuffmanCode::W_HAND_GOLD,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Error)]
pub enum HuffmanCodeForFieldError {
    #[error("huffman code for field error. undecided yet.")]
    UndecidedYet,
    #[error("huffman code for field error. max bit length.")]
    OverMaxBitLength,
}

#[derive(Debug, Error)]
pub enum HuffmanCodeForHandError {
    #[error("huffman code for hand error. undecided yet.")]
    UndecidedYet,
    #[error("huffman code for hand error. max bit length.")]
    OverMaxBitLength,
}

impl std::convert::TryFrom<&HuffmanCode> for Piece {
    type Error = HuffmanCodeForFieldError;
    fn try_from(hc: &HuffmanCode) -> Result<Self, Self::Error> {
        match *hc {
            HuffmanCode::EMPTY => Ok(Piece::EMPTY),
            HuffmanCode::B_PAWN => Ok(Piece::B_PAWN),
            HuffmanCode::B_LANCE => Ok(Piece::B_LANCE),
            HuffmanCode::B_KNIGHT => Ok(Piece::B_KNIGHT),
            HuffmanCode::B_SILVER => Ok(Piece::B_SILVER),
            HuffmanCode::B_BISHOP => Ok(Piece::B_BISHOP),
            HuffmanCode::B_ROOK => Ok(Piece::B_ROOK),
            HuffmanCode::B_GOLD => Ok(Piece::B_GOLD),
            HuffmanCode::B_PRO_PAWN => Ok(Piece::B_PRO_PAWN),
            HuffmanCode::B_PRO_LANCE => Ok(Piece::B_PRO_LANCE),
            HuffmanCode::B_PRO_KNIGHT => Ok(Piece::B_PRO_KNIGHT),
            HuffmanCode::B_PRO_SILVER => Ok(Piece::B_PRO_SILVER),
            HuffmanCode::B_HORSE => Ok(Piece::B_HORSE),
            HuffmanCode::B_DRAGON => Ok(Piece::B_DRAGON),
            HuffmanCode::W_PAWN => Ok(Piece::W_PAWN),
            HuffmanCode::W_LANCE => Ok(Piece::W_LANCE),
            HuffmanCode::W_KNIGHT => Ok(Piece::W_KNIGHT),
            HuffmanCode::W_SILVER => Ok(Piece::W_SILVER),
            HuffmanCode::W_BISHOP => Ok(Piece::W_BISHOP),
            HuffmanCode::W_ROOK => Ok(Piece::W_ROOK),
            HuffmanCode::W_GOLD => Ok(Piece::W_GOLD),
            HuffmanCode::W_PRO_PAWN => Ok(Piece::W_PRO_PAWN),
            HuffmanCode::W_PRO_LANCE => Ok(Piece::W_PRO_LANCE),
            HuffmanCode::W_PRO_KNIGHT => Ok(Piece::W_PRO_KNIGHT),
            HuffmanCode::W_PRO_SILVER => Ok(Piece::W_PRO_SILVER),
            HuffmanCode::W_HORSE => Ok(Piece::W_HORSE),
            HuffmanCode::W_DRAGON => Ok(Piece::W_DRAGON),
            HuffmanCode {
                bit_length: HuffmanCode::MAX_BIT_LENGTH_FOR_FIELD..=std::u8::MAX,
                ..
            } => Err(Self::Error::OverMaxBitLength),
            _ => Err(Self::Error::UndecidedYet),
        }
    }
}

pub type ColorAndPieceTypeForHand = (Color, PieceType);
impl std::convert::TryFrom<&HuffmanCode> for ColorAndPieceTypeForHand {
    type Error = HuffmanCodeForHandError;
    fn try_from(hc: &HuffmanCode) -> Result<Self, Self::Error> {
        match *hc {
            HuffmanCode::B_HAND_PAWN => Ok((Color::BLACK, PieceType::PAWN)),
            HuffmanCode::W_HAND_PAWN => Ok((Color::BLACK, PieceType::PAWN)),
            HuffmanCode::B_HAND_LANCE => Ok((Color::BLACK, PieceType::LANCE)),
            HuffmanCode::W_HAND_LANCE => Ok((Color::BLACK, PieceType::LANCE)),
            HuffmanCode::B_HAND_KNIGHT => Ok((Color::BLACK, PieceType::KNIGHT)),
            HuffmanCode::W_HAND_KNIGHT => Ok((Color::BLACK, PieceType::KNIGHT)),
            HuffmanCode::B_HAND_SILVER => Ok((Color::BLACK, PieceType::SILVER)),
            HuffmanCode::W_HAND_SILVER => Ok((Color::BLACK, PieceType::SILVER)),
            HuffmanCode::B_HAND_GOLD => Ok((Color::BLACK, PieceType::GOLD)),
            HuffmanCode::W_HAND_GOLD => Ok((Color::BLACK, PieceType::GOLD)),
            HuffmanCode::B_HAND_BISHOP => Ok((Color::BLACK, PieceType::BISHOP)),
            HuffmanCode::W_HAND_BISHOP => Ok((Color::BLACK, PieceType::BISHOP)),
            HuffmanCode::B_HAND_ROOK => Ok((Color::BLACK, PieceType::ROOK)),
            HuffmanCode::W_HAND_ROOK => Ok((Color::BLACK, PieceType::ROOK)),
            HuffmanCode {
                bit_length: HuffmanCode::MAX_BIT_LENGTH_FOR_HAND..=std::u8::MAX,
                ..
            } => Err(Self::Error::OverMaxBitLength),
            _ => Err(Self::Error::UndecidedYet),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HuffmanCodedPosition {
    pub buf: [u8; 32],
    pub ply: i16,
}

impl HuffmanCodedPosition {
    pub fn from(pos: &Position) -> HuffmanCodedPosition {
        let mut hcp = HuffmanCodedPosition {
            buf: [0; 32],
            ply: pos.ply() as i16,
        };
        let mut bs = BitStreamWriter::new(&mut hcp.buf);
        bs.put_bit_from_lsb(pos.side_to_move().0 as u8);
        bs.put_bits_from_lsb(pos.king_square(Color::BLACK).0 as u8, 7);
        bs.put_bits_from_lsb(pos.king_square(Color::WHITE).0 as u8, 7);
        for &sq in Square::ALL.iter() {
            let pc = pos.piece_on(sq);
            if pc == Piece::B_KING || pc == Piece::W_KING {
                continue;
            }
            let hc = HuffmanCode::new(pc);
            bs.put_bits_from_lsb(hc.value, hc.bit_length as usize);
        }
        for &c in Color::ALL.iter() {
            let hand = pos.hand(c);
            for &pt in PieceType::ALL_HAND.iter() {
                let hc = HuffmanCode::new_from_color_and_hand_piece_type(c, pt);
                for _ in 0..hand.num(pt) as usize {
                    bs.put_bits_from_lsb(hc.value, hc.bit_length as usize);
                }
            }
        }
        hcp
    }
}

pub struct BitStreamReader<'a> {
    pub slice: &'a [u8],
    pub current_index: usize,
    pub current_bit: usize,
}

pub struct BitStreamWriter<'a> {
    pub slice: &'a mut [u8],
    pub current_index: usize,
    pub current_bit: usize,
}

impl<'a> BitStreamReader<'a> {
    pub fn new(buf: &[u8]) -> BitStreamReader {
        BitStreamReader {
            slice: buf,
            current_index: 0,
            current_bit: 0,
        }
    }
    pub fn get_bit_from_lsb(&mut self) -> u8 {
        let bit = if (self.slice[self.current_index] & (1 << self.current_bit)) == 0 {
            0
        } else {
            1
        };
        self.current_bit += 1;
        if self.current_bit == 8 {
            self.current_index += 1;
            self.current_bit = 0;
        }
        bit
    }
    pub fn get_bits_from_lsb(&mut self, bit_length: usize) -> u8 {
        let mut bits = 0;
        for i in 0..bit_length {
            bits |= self.get_bit_from_lsb() << i;
        }
        bits
    }
}

impl<'a> BitStreamWriter<'a> {
    fn new(buf: &mut [u8]) -> BitStreamWriter {
        BitStreamWriter {
            slice: buf,
            current_index: 0,
            current_bit: 0,
        }
    }
    fn put_bit_from_lsb(&mut self, bit: u8) {
        debug_assert!(bit == 0 || bit == 1);
        self.slice[self.current_index] |= bit << self.current_bit;
        self.current_bit += 1;
        if self.current_bit == 8 {
            self.current_index += 1;
            self.current_bit = 0;
        }
    }
    fn put_bits_from_lsb(&mut self, bits: u8, bit_length: usize) {
        let mut bits = bits;
        for _ in 0..bit_length {
            let bit = bits & 1;
            bits >>= 1;
            self.put_bit_from_lsb(bit);
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum GameResult {
    Draw,
    BlackWin,
    WhiteWin,
}

#[repr(C)]
pub struct HuffmanCodedPositionAndEval {
    pub hcp: HuffmanCodedPosition,
    pub value: i16,
    pub best_move16: u16,
    pub end_ply: i16,
    pub game_result: GameResult,
    pub padding: u8,
}

#[test]
fn test_huffmancodedpositionandeval_size() {
    assert_eq!(std::mem::size_of::<GameResult>(), 1);
    assert_eq!(std::mem::size_of::<HuffmanCodedPositionAndEval>(), 42);
}
