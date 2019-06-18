use crate::types::*;

pub const START_SFEN: &str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";

#[derive(Debug)]
pub enum SfenError {
    InvalidNumberOfSections { sections: usize },
    InvalidNumberOfFiles { files: usize },
    InvalidNumberOfRanks { ranks: usize },
    InvalidNumberOfEmptySquares { empty_squares: i64 },
    InvalidPieceCharactors { chars: String },
    InvalidHandPieceCharactors { chars: String },
    InvalidNumberOfHandPieces { number: i64 },
    InvalidNumberOfPawns { number: i64 },
    InvalidNumberOfLances { number: i64 },
    InvalidNumberOfKnights { number: i64 },
    InvalidNumberOfSilvers { number: i64 },
    InvalidNumberOfGolds { number: i64 },
    InvalidNumberOfBishops { number: i64 },
    InvalidNumberOfRooks { number: i64 },
    InvalidSideToMoveCharactors { chars: String },
    InvalidGamePly { chars: String },
    SameHandPieceTwice { pt: PieceType },
    KingIsNothing { c: Color },
}
