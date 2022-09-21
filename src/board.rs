use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ChessBoard {
    layout: [[Option<ChessPiece>; 8]; 8],
}

#[derive(Clone, Copy, Debug)]
struct ChessPiece {
    color: Color,
    piece: Pieces
}
impl ChessPiece {
    fn pawn(color: Color) -> Self {
        Self { color, piece: Pieces::Pawn }
    }
    fn knight(color: Color) -> ChessPiece {
        Self { color, piece: Pieces::Knight }
    }
    fn bishop(color: Color) -> ChessPiece {
        Self { color, piece: Pieces::Bishop }
    }
    fn rook(color: Color) -> ChessPiece {
        Self { color, piece: Pieces::Rook }
    }
    fn queen(color: Color) -> ChessPiece {
        Self { color, piece: Pieces::Queen }
    }
    fn king(color: Color) -> ChessPiece {
        Self { color, piece: Pieces::King }
    }
}

#[derive(Clone, Copy, Debug)]
enum Color {
    White,
    Black
}

#[derive(Clone, Copy, Debug)]
enum Pieces {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl ChessBoard {
    pub fn new() -> Self {
        Self {
            layout: [Self::back_rank(Color::White),
                     [Some(ChessPiece::pawn(Color::White)); 8],
                     [None; 8],
                     [None; 8],
                     [None; 8],
                     [None; 8],
                     [Some(ChessPiece::pawn(Color::Black)); 8],
                     Self::back_rank(Color::Black)],
        }
    }

    fn back_rank(color: Color) -> [Option<ChessPiece>; 8] {
        [Some(ChessPiece::rook(color)), Some(ChessPiece::knight(color)), Some(ChessPiece::bishop(color)), Some(ChessPiece::queen(color)), Some(ChessPiece::king(color)), Some(ChessPiece::bishop(color)), Some(ChessPiece::knight(color)), Some(ChessPiece::rook(color))]
    }
}