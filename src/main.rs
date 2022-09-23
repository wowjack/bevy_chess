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
    let board_info = BoardInfo::default();
    let board_size = board_info.size;
    commands.spawn()
            .insert(board_info)
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(-200., -200., 1.),
                    scale: Vec3::new(1., 1., 1.),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                for y in 0..8 as u8 {
                    for x in 0..8 as u8 {
                        parent.spawn_bundle(BoardTile::make_sprite(x, y, board_size))
                        .insert(BoardTile::new(x, y));
                    }
                    //spawn black pawns
                    parent.spawn()
                        .insert(ChessPiece::pawn(ChessColor::Black, (y, 7)))
                        .insert_bundle(SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new((board_size as f32/8. * y as f32) as f32, (board_size as f32/8. * 6.) as f32, 1.),
                                scale: Vec3::new(board_size as f32/12., board_size as f32/12., 1.),
                                ..default()
                            },
                            ..default()
                        });
                    //spawn white pawns
                    parent.spawn()
                        .insert(ChessPiece::pawn(ChessColor::White, (y, 8)))
                        .insert_bundle(SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new((board_size as f32/8. * y as f32) as f32, (board_size as f32/8. * 1.) as f32, 1.),
                                scale: Vec3::new(board_size as f32/12., board_size as f32/12., 1.),
                                ..default()
                            },
                            ..default()
                        });
                }
                
            });
}


