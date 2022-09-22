use bevy::prelude::*;

mod board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .add_startup_system(add_board_tiles)
        .run();
}

fn init(mut commands: Commands) {
    let board = board::ChessBoard::new();
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn().insert(board);
}

fn add_board_tiles(mut commands: Commands) {
    for y in 0..8 as u8 {
        for x in 0..8 as u8 {
            let c = if x+y%2==0 {Color::BLACK} else {Color::BEIGE};
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: c,
                    ..Sprite::default()
                },
                transform: Transform { translation: Vec3::new((x*10) as f32, (y*10) as f32, 0.),
                                       scale: Vec3::new(10., 10., 1.),
                                       ..Transform::default() 
                },
                ..SpriteBundle::default()
            });
        }
    }
}

