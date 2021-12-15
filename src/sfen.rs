use crate::types::*;
use thiserror::Error;

pub const START_SFEN: &str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";

#[rustfmt::skip]
#[derive(Debug, Error)]
pub enum SfenError {
    #[error("invalid number of sections. found {sections} sections.")]
    InvalidNumberOfSections { sections: usize },
    #[error("invalid number of files. found {files} files.")]
    InvalidNumberOfFiles { files: usize },
    #[error("invalid number of ranks. found {ranks} ranks.")]
    InvalidNumberOfRanks { ranks: usize },
    #[error("invalid number of empty squares. found {empty_squares} empty squares.")]
    InvalidNumberOfEmptySquares { empty_squares: i64 },
    #[error(r##"invalid piece charactors. found "{token}"."##)]
    InvalidPieceCharactors { token: String },
    #[error(r##"invalid hand piece charactors. found "{token}"."##)]
    InvalidHandPieceCharactors { token: String },
    #[error("invalid number of hand pieces. found {number}.")]
    InvalidNumberOfHandPieces { number: i64 },
    #[error(r##"end with hand piece num "{last_number}"."##)]
    EndWithHandPieceNumber { last_number: i64 },
    #[error("invalid number of pawns. found {number}.")]
    InvalidNumberOfPawns { number: i64 },
    #[error("invalid number of lances. found {number}.")]
    InvalidNumberOfLances { number: i64 },
    #[error("invalid number of knights. found {number}.")]
    InvalidNumberOfKnights { number: i64 },
    #[error("invalid number of silvers. found {number}.")]
    InvalidNumberOfSilvers { number: i64 },
    #[error("invalid number of golds. found {number}.")]
    InvalidNumberOfGolds { number: i64 },
    #[error("invalid number of bishops. found {number}.")]
    InvalidNumberOfBishops { number: i64 },
    #[error("invalid number of rooks. found {number}.")]
    InvalidNumberOfRooks { number: i64 },
    #[error("invalid side to move charactors. found {chars}.")]
    InvalidSideToMoveCharactors { chars: String },
    #[error("invalid game ply. found {chars}.")]
    InvalidGamePly { chars: String },
    #[error(r##"same hand piece twice. found "{token}"."##)]
    SameHandPieceTwice { token: String },
    #[error("{c:?} king is nothing.")]
    KingIsNothing { c: Color },
}
