use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ChessBoard {
    pieces: Vec<ChessPiece>
}

impl ChessBoard {
    pub fn new() -> Self {
        let mut p = Vec::new();
        
        for i in 1..=8 {
            p.push(ChessPiece::pawn(Color::White, (i, 2)));
            p.push(ChessPiece::pawn(Color::Black, (i, 7)));
            match i {
                1 | 8 => { p.push(ChessPiece::rook(Color::White, (i, 1))); p.push(ChessPiece::rook(Color::Black, (i, 8))) },
                2 | 7 => { p.push(ChessPiece::knight(Color::White, (i, 1))); p.push(ChessPiece::knight(Color::Black, (i, 8))) },
                3 | 6 => { p.push(ChessPiece::bishop(Color::White, (i, 1))); p.push(ChessPiece::bishop(Color::Black, (i, 8))) },
                4 => { p.push(ChessPiece::queen(Color::White, (i, 1))); p.push(ChessPiece::queen(Color::Black, (i, 8))) },
                5 => { p.push(ChessPiece::king(Color::White, (i, 1))); p.push(ChessPiece::king(Color::Black, (i, 8))) },
                _ => panic!("Failed to create new chess board")
            }
        }
        
        Self{ pieces: p }
    }
}

#[derive(Clone, Copy, Debug, Component)]
struct ChessPiece {
    color: Color,
    piece: Pieces,
    position: (u8, u8)
}
impl ChessPiece {
    fn pawn(color: Color, position: (u8, u8)) -> Self {
        Self { color, piece: Pieces::Pawn, position }
    }
    fn knight(color: Color, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::Knight, position }
    }
    fn bishop(color: Color, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::Bishop, position }
    }
    fn rook(color: Color, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::Rook, position }
    }
    fn queen(color: Color, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::Queen, position }
    }
    fn king(color: Color, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::King, position }
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

