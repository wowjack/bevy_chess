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

fn spawn_new_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    use crate::board::*;

    let board_info = BoardInfo {
        size: 600,
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
                    let b_pawn: Handle<Image> = asset_server.load("b_pawn_1x_ns.png");
                    let w_pawn: Handle<Image> = asset_server.load("w_pawn_1x_ns.png");
                    let b_rook: Handle<Image> = asset_server.load("b_rook_1x_ns.png");
                    let w_rook: Handle<Image> = asset_server.load("w_rook_1x_ns.png");
                    let b_knight: Handle<Image> = asset_server.load("b_knight_1x_ns.png");
                    let w_knight: Handle<Image> = asset_server.load("w_knight_1x_ns.png");
                    let b_bishop: Handle<Image> = asset_server.load("b_bishop_1x_ns.png");
                    let w_bishop: Handle<Image> = asset_server.load("w_bishop_1x_ns.png");
                    let b_queen: Handle<Image> = asset_server.load("b_queen_1x_ns.png");
                    let w_queen: Handle<Image> = asset_server.load("w_queen_1x_ns.png");
                    let b_king: Handle<Image> = asset_server.load("b_king_1x_ns.png");
                    let w_king: Handle<Image> = asset_server.load("w_king_1x_ns.png");
                    //spawn black pawns
                    parent.spawn()
                        .insert(ChessPiece::pawn(ChessColor::Black, (y, 6)))
                        .insert_bundle(ChessPiece::piece_sprite(ChessColor::Black, (y,6), board_size, b_pawn));
                    //spawn white pawns
                    parent.spawn()
                        .insert(ChessPiece::pawn(ChessColor::White, (y, 1)))
                        .insert_bundle(ChessPiece::piece_sprite(ChessColor::White, (y,1), board_size, w_pawn));
                    match y {
                        0 | 7 => {
                            parent.spawn()
                                .insert(ChessPiece::rook(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::Black, (y,7), board_size, b_rook));
                            parent.spawn()
                                .insert(ChessPiece::rook(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::White, (y,0), board_size, w_rook));
                            },
                        1 | 6 => {
                            parent.spawn()
                                .insert(ChessPiece::knight(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::Black, (y,7), board_size, b_knight));
                            parent.spawn()
                                .insert(ChessPiece::knight(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::White, (y,0), board_size, w_knight));
                            },
                        2 | 5 => {
                            parent.spawn()
                                .insert(ChessPiece::bishop(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::Black, (y,7), board_size, b_bishop));
                            parent.spawn()
                                .insert(ChessPiece::bishop(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::White, (y,0), board_size, w_bishop));
                            },
                        3 => {
                            parent.spawn()
                                .insert(ChessPiece::queen(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::Black, (y,7), board_size, b_queen));
                            parent.spawn()
                                .insert(ChessPiece::queen(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::White, (y,0), board_size, w_queen));
                            },
                        4 => {
                            parent.spawn()
                                .insert(ChessPiece::king(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::Black, (y,7), board_size, b_king));
                            parent.spawn()
                                .insert(ChessPiece::king(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite(ChessColor::White, (y,0), board_size, w_king));
                            },
                        _ => panic!("Invalid chess piece spawn.")
                    }
                }
            });
}


