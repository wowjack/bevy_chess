use bevy::prelude::{*, Color};

#[derive(Clone, Copy, Component)]
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
enum ChessColor {
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

#[derive(Component)]
pub(crate) struct BoardInfo{
    pub size: u16,
    pub position: Vec2
}
impl BoardInfo {
    pub(crate) fn default() -> Self {
        Self {
            size: 500,
            position: Vec2::new(0., 0.)
        }
    }
}

#[derive(Component)]
pub(crate) struct BoardTile {
    pub position: (u8, u8)
}
impl BoardTile {
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self { position: (x, y) }
    }
    pub(crate) fn make_sprite(x: u8, y: u8, board_size: u16) -> SpriteBundle {
        let color = if (x+y)%2==0 {Color::BLACK} else {Color::BEIGE};
        SpriteBundle {
            sprite: Sprite {
                color,
                ..Sprite::default()
            },
            transform: Transform {
                translation: Vec3::new((board_size/8 * x as u16) as f32, (board_size/8 * y as u16) as f32, 1.),
                scale: Vec3::new(board_size as f32/8., board_size as f32/8., 1.),
                ..Transform::default()
            },
            ..SpriteBundle::default()
        }
    }
}
