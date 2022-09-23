use bevy::prelude::*;

mod board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .add_startup_system(spawn_new_board)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_new_board(mut commands: Commands) {
    use crate::board::*;
    let board_info = BoardInfo {
        size: 200,
        position: Vec2::new(-300., -300.)
    };
    let board_size = board_info.size;
    commands.spawn()
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(board_info.position.x, board_info.position.y, 1.),
                    scale: Vec3::new(1., 1., 1.),
                    ..default()
                },
                ..default()
            })
            .insert(board_info)
            .with_children(|parent| {
                for y in 0..8 as u8 {
                    for x in 0..8 as u8 {
                        parent.spawn_bundle(BoardTile::make_sprite(x, y, board_size))
                        .insert(BoardTile::new(x, y));
                    }
                    //spawn black pawns
                    parent.spawn()
                        .insert(ChessPiece::pawn(ChessColor::Black, (y, 6)))
                        .insert_bundle(ChessPiece::pawn_sprite(ChessColor::Black, (y,6), board_size));
                    //spawn white pawns
                    parent.spawn()
                        .insert(ChessPiece::pawn(ChessColor::White, (y, 6)))
                        .insert_bundle(ChessPiece::pawn_sprite(ChessColor::White, (y,1), board_size));
                }
            });
}


