use bevy::prelude::*;
use board::{SelectedPiece, ChessPiece};

mod board;
mod common;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<board::PieceClickedEvent>()
        .add_startup_system(init)
        .add_startup_system(spawn_new_board)
        .add_system(click_pieces)
        .add_system(set_selected_piece)
        .run();
}

fn set_selected_piece(
    mut click_event: EventReader<board::PieceClickedEvent>,
    selected_query: Query<&SelectedPiece>,
    pieces_query: Query<&ChessPiece>
) {
    let mut clicked_pos: Option<(u8, u8)> = None;
    for e in click_event.iter() {
        clicked_pos = Some(e.position);
    }

    let mut clicked_piece: Option<&ChessPiece> = None;
    match clicked_pos {
        None => return,
        Some(pos) => {
            for piece in pieces_query.iter() {
                if piece.position == pos {
                    clicked_piece = Some(piece);
                    break;
                }
            }
        }
    }

    if let Some(piece) = clicked_piece {
        println!("clicked piece {:?} {:?} at {}", piece.color, piece.piece, pos_to_str(piece.position));
    }
}

fn click_pieces(
    piece_query: Query<(&board::ChessPiece, &GlobalTransform, &Sprite)>,
    camera_query: Query<(&Camera, &GlobalTransform), Without<board::ChessPiece>>,
    mut piece_click_event_writer: EventWriter<board::PieceClickedEvent>,
    mouse_buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    let mut cursor_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => Vec2::default()
    };
    cursor_pos.x -= window.width()/2.;
    cursor_pos.y -= window.height()/2.;

    if mouse_buttons.just_pressed(MouseButton::Left) {
        let pos = common::mouse_pos_to_global(window, camera_query.single());
        for (piece, transform, sprite) in piece_query.iter() {
            let piece_info = transform.to_scale_rotation_translation();
            let sprite_width = sprite.custom_size.unwrap().x;
            let sprite_height = sprite.custom_size.unwrap().y;
            if pos.x > piece_info.2.x-sprite_width/2. &&
                pos.x < piece_info.2.x+sprite_width/2. &&
                pos.y > piece_info.2.y-sprite_height/2. &&
                pos.y < piece_info.2.y+sprite_height/2. 
            {
                piece_click_event_writer.send(board::PieceClickedEvent { position: piece.position });
                return;
            }
        }
    }
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_new_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    use crate::board::*;

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
                    //spawn black pawns
                    parent.spawn()
                        .insert(ChessPiece::pawn(ChessColor::Black, (y, 6)))
                        .insert_bundle(ChessPiece::piece_sprite((y,6), board_size, b_pawn.clone()));
                    //spawn white pawns
                    parent.spawn()
                        .insert(ChessPiece::pawn(ChessColor::White, (y, 1)))
                        .insert_bundle(ChessPiece::piece_sprite((y,1), board_size, w_pawn.clone()));
                    match y {
                        0 | 7 => {
                            parent.spawn()
                                .insert(ChessPiece::rook(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite((y,7), board_size, b_rook.clone()));
                            parent.spawn()
                                .insert(ChessPiece::rook(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite((y,0), board_size, w_rook.clone()));
                            },
                        1 | 6 => {
                            parent.spawn()
                                .insert(ChessPiece::knight(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite((y,7), board_size, b_knight.clone()));
                            parent.spawn()
                                .insert(ChessPiece::knight(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite((y,0), board_size, w_knight.clone()));
                            },
                        2 | 5 => {
                            parent.spawn()
                                .insert(ChessPiece::bishop(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite((y,7), board_size, b_bishop.clone()));
                            parent.spawn()
                                .insert(ChessPiece::bishop(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite((y,0), board_size, w_bishop.clone()));
                            },
                        3 => {
                            parent.spawn()
                                .insert(ChessPiece::queen(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite((y,7), board_size, b_queen.clone()));
                            parent.spawn()
                                .insert(ChessPiece::queen(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite((y,0), board_size, w_queen.clone()));
                            },
                        4 => {
                            parent.spawn()
                                .insert(ChessPiece::king(ChessColor::Black, (y, 7)))
                                .insert_bundle(ChessPiece::piece_sprite((y,7), board_size, b_king.clone()));
                            parent.spawn()
                                .insert(ChessPiece::king(ChessColor::White, (y, 0)))
                                .insert_bundle(ChessPiece::piece_sprite((y,0), board_size, w_king.clone()));
                            },
                        _ => panic!("Invalid chess piece spawn.")
                    }
                }
            });
    commands.spawn().insert(board::SelectedPiece {
        piece: None,
        transform: None
    });
}

fn pos_to_str(pos: (u8, u8)) -> String {
    let mut str = String::from(
        match pos.0 { 0 => "a", 1 => "b", 2 => "c", 3 => "d", 4 => "e", 5 => "f", 6 => "g", 7 => "h", _ => "err" }
    );
    str.push_str(
        match pos.1 { 0 => "1", 1 => "2", 2 => "3", 3 => "4", 4 => "5", 5 => "6", 6 => "7", 7 => "8", _ => "err" }
    );
    return str;
}