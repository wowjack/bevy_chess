use bevy::prelude::{*, Color};

#[derive(Clone, Copy, Component)]
pub struct ChessPiece {
    pub color: ChessColor,
    pub piece: Pieces,
    pub position: (u8, u8)
}
impl ChessPiece {
    pub fn piece_sprite(position: (u8, u8), board_size: u16, texture: Handle<Image>) -> SpriteBundle {
        SpriteBundle {
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(board_size as f32/12., board_size as f32/12.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(board_size as f32/8. * position.0 as f32, board_size as f32/8. * position.1 as f32, 1.),
                ..default()
            },
            ..default()
        }
    }
    pub fn pawn(color: ChessColor, position: (u8, u8)) -> Self {
        Self { color, piece: Pieces::Pawn, position }
    }
    pub fn knight(color: ChessColor, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::Knight, position }
    }
    pub fn bishop(color: ChessColor, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::Bishop, position }
    }
    pub fn rook(color: ChessColor, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::Rook, position }
    }
    pub fn queen(color: ChessColor, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::Queen, position }
    }
    pub fn king(color: ChessColor, position: (u8, u8)) -> ChessPiece {
        Self { color, piece: Pieces::King, position }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ChessColor {
    White,
    Black
}

#[derive(Clone, Copy, Debug)]
pub enum Pieces {
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
#[allow(unused)]
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
                translation: Vec3::new(board_size as f32/8. * x as f32, board_size as f32/8. * y as f32, 1.),
                scale: Vec3::new(board_size as f32/8., board_size as f32/8., 1.),
                ..Transform::default()
            },
            ..SpriteBundle::default()
        }
    }
}

#[derive(Component)]
pub(crate) struct SelectedPiece {
    pub piece: Option<&'static ChessPiece>,
    pub transform: Option<&'static mut Transform>
}

pub(crate) struct PieceClickedEvent {
    pub position: (u8, u8)
}