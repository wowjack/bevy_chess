use bevy::prelude::*;

mod board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    let board = board::ChessBoard::new();
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn().insert(board);
}   

