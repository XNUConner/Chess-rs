use speedy2d::{Window, Graphics2D};
use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::window::{
    WindowHandler,
    WindowHelper,
    WindowStartupInfo,
};
use speedy2d::dimen::{Vector2, UVec2, Vec2};
use speedy2d::shape::{Rectangle, URect};
use speedy2d::color::Color;

use std::collections::HashMap;


fn main() {

    env_logger::init();

    let (window_width, window_height) = (800, 800);

    let window = Window::new_centered("Chess", (window_height, window_width)).unwrap();

    window.run_loop( GameWindowHandler::new() );

}

struct GameWindowHandler {
    game: Chess,
    renderer: Option<Renderer>,
}

impl GameWindowHandler {
    fn new() -> Self {
        GameWindowHandler {
            game: Chess::new(),
            renderer: None,
        }
    }
}

impl WindowHandler for GameWindowHandler {
    fn on_start(&mut self, helper: &mut WindowHelper, info: WindowStartupInfo) {
        log::info!("Got on_start callback: {:?}", info);
        self.renderer = Some(Renderer::new(*info.viewport_size_pixels()));
        //self.game.load_fen("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8");
        self.game.pieces[7] = ChessPiece::KNIGHT as u8 | ChessColor::BLACK as u8;
        self.game.pieces[0] = ChessPiece::KNIGHT as u8 | ChessColor::WHITE as u8;
        self.game.pieces[8] = ChessPiece::KING   as u8 | ChessColor::WHITE as u8;
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper, size_pixels: UVec2) {
        log::info!("Got on_resize callback: {:?}", size_pixels);
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {

        if self.renderer.as_ref().unwrap().piece_images.is_none() {
            self.renderer.as_mut().unwrap().load_images(graphics);
        }

        // Clear the screen
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.renderer.as_mut().unwrap().render_board(&self.game.pieces, graphics);
    }
}

#[derive(Debug)]
struct Renderer {
    window_rect: URect,
    game_rect: URect,
    piece_images: Option<HashMap<ChessPiece, [ImageHandle; 2]>>,
}

impl Renderer {
    fn new(window_size: UVec2) -> Self {

        const GAME_WINDOW_SIZE_PX_Y: u32 = 700;
        const GAME_WINDOW_SIZE_PX_X: u32 = 700;

        let game_topleft_y = (window_size.y - GAME_WINDOW_SIZE_PX_Y) / 2;
        let game_topleft_x = (window_size.x - GAME_WINDOW_SIZE_PX_X) / 2;
        let game_bottomright_y = game_topleft_y + GAME_WINDOW_SIZE_PX_Y;
        let game_bottomright_x = game_topleft_x + GAME_WINDOW_SIZE_PX_X;


        Renderer {
            window_rect: URect::new( UVec2::new(0, 0),                             UVec2::new(window_size.x, window_size.y) ),
            game_rect:   URect::new( UVec2::new(game_topleft_x, game_topleft_y),   UVec2::new(game_bottomright_x, game_bottomright_y) ),
            piece_images: None,
        }
    }

    fn load_images(&mut self, graphics: &mut Graphics2D) {
        self.piece_images = Some( HashMap::from([
            (ChessPiece::PAWN,   Self::load_images_for_piece(ChessPiece::PAWN,   graphics)),
            (ChessPiece::ROOK,   Self::load_images_for_piece(ChessPiece::ROOK,   graphics)),
            (ChessPiece::KNIGHT, Self::load_images_for_piece(ChessPiece::KNIGHT, graphics)),
            (ChessPiece::BISHOP, Self::load_images_for_piece(ChessPiece::BISHOP, graphics)),
            (ChessPiece::QUEEN,  Self::load_images_for_piece(ChessPiece::QUEEN,  graphics)),
            (ChessPiece::KING,   Self::load_images_for_piece(ChessPiece::KING,   graphics)),
        ]));
    }

    fn load_images_for_piece(piece: ChessPiece, graphics: &mut Graphics2D) -> [ImageHandle; 2] {

        const IMAGE_DIR: &str = "img/";
        const IMAGE_EXT: &str = ".png";

        let piece_char = match piece {
            ChessPiece::PAWN   => 'p',
            ChessPiece::ROOK   => 'r',
            ChessPiece::KNIGHT => 'n',
            ChessPiece::BISHOP => 'b',
            ChessPiece::QUEEN  => 'q',
            ChessPiece::KING   => 'k',
        };

        let light_char = 'l';
        let dark_char  = 'd';
        let path_light = format!("{IMAGE_DIR}{piece_char}{light_char}{IMAGE_EXT}");
        let path_dark  = format!("{IMAGE_DIR}{piece_char}{dark_char}{IMAGE_EXT}");


        [
            graphics.create_image_from_file_path(None, ImageSmoothingMode::NearestNeighbor, path_light.clone()).expect( &format!("Failed to load image for {}", path_light).to_owned()),
            graphics.create_image_from_file_path(None, ImageSmoothingMode::NearestNeighbor, path_dark.clone()).expect( &format!("Failed to load image for {}", path_dark).to_owned()),
        ]
    } 

    fn render_board(&self, pieces: &[u8], graphics: &mut Graphics2D) {

        // Prepare the dimensions and coordinates of the board square to be drawn

        // square_size:         Used for the Height and Width of the square
        // square_top_left:     Top-left corner x,y coordinates of the square
        // square_bottom_right: Bottom-right corner x,y coordinates of the square
 
        let square_size = (self.game_rect.bottom_right().y - self.game_rect.top_left().y) as f32 / 8.0; 
        let square_top_left = Vec2::new(self.game_rect.top_left().x as f32, self.game_rect.top_left().y as f32);
        let square_bottom_right = Vec2::new(square_top_left.x + square_size, square_top_left.y + square_size);


        for y in 0..8 {
            for x in 0..8 {
                let y_offset = y as f32 * square_size;
                let x_offset = x as f32 * square_size;
                
                let quad = [
                    // from origin
                    Vec2::new(square_top_left.y + y_offset, square_bottom_right.x + x_offset),
                    Vec2::new(square_bottom_right.y + y_offset, square_bottom_right.x + x_offset),
                    Vec2::new(square_bottom_right.y + y_offset, square_top_left.x + x_offset),
                    Vec2::new(square_top_left.y + y_offset, square_top_left.x + x_offset),
                ];

                let square_color = match (y + x) % 2 {
                    0 => Color::WHITE,
                    _ => Color::from_int_rgb(200, 200, 200),
                };

		graphics.draw_quad(quad, square_color);

                let board_position = (y * 8) + x;
                let piece = pieces[board_position];

                // Skip if no piece at that position
                if piece == 0 { continue; }

                // extract color
                let color = match piece > 128 {
                    true  => ChessColor::BLACK,
                    false => ChessColor::WHITE,
                };

                let name = ChessPiece::try_from(piece ^ (color as u8)).unwrap();

                let imagehandle = match color {
                    ChessColor::WHITE => &self.piece_images.as_ref().unwrap().get(&name).unwrap()[0],
                    ChessColor::BLACK => &self.piece_images.as_ref().unwrap().get(&name).unwrap()[1],
                };

                let rect = Rectangle::new( 
                    Vector2::new(square_top_left.y as f32 + y_offset, square_top_left.x as f32 + x_offset), 
                    Vector2::new(square_bottom_right.y as f32 + y_offset, square_bottom_right.x as f32 + x_offset),
                );

                graphics.draw_rectangle_image(&rect, imagehandle);
                
            }
        }
        

    }
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[repr(u8)]
enum ChessColor { // PieceColor
    WHITE = 64,
    BLACK = 128,
}

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[repr(u8)]
enum ChessPiece { // PieceName
    PAWN = 1,
    ROOK = 2,
    KNIGHT = 4,
    BISHOP = 8,
    QUEEN = 16,
    KING = 32,
}

impl TryFrom<u8> for ChessPiece {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
       match val {
            1  => Ok(Self::PAWN),
            2  => Ok(Self::ROOK),
            4  => Ok(Self::KNIGHT),
            8  => Ok(Self::BISHOP),
            16 => Ok(Self::QUEEN),
            32 => Ok(Self::KING),
            _  => Err("Invalid u8 provided during ChessPiece conversion"),
       }
    }
}

struct Player {
    color: ChessColor,
    is_turn: bool,
    is_in_check: bool,
}

impl Player {
    fn new(color: ChessColor) -> Self {
        let is_turn = color == ChessColor::WHITE;

        Player {
            color,
            is_turn,
            is_in_check: false,
        }

    }
}

struct Chess {
    pieces: [u8; 64], // Change to 'board'
    display_rect: Rectangle,
    white: Player,
    black: Player,
}

impl Chess {
    fn new() -> Self {
        Chess {
            pieces: [0; 64], // change to 'board'
            display_rect: Rectangle::new( Vector2::new(100.0, 100.0), Vector2::new(200.0, 200.0) ),
            white: Player::new(ChessColor::WHITE),
            black: Player::new(ChessColor::BLACK),
        }
    }

    fn load_fen(&mut self, fen: &str) {
        assert!( fen.is_ascii() );

        let mut board_ptr: usize = 0;
        for ch in fen.chars() {

            // Skip '/'
            if ch == '/' { continue; }

            assert!( ch.is_ascii_alphanumeric() );

            // For digits, increment board_ptr by digit, continue
            if let Some(digit) = ch.to_digit(/* Radix */ 10) {
                assert!(digit > 0 && digit <= 8);
                board_ptr += digit as usize;
                continue;
            }

            // For chars setup the corresponding black or white piece at that square
            assert!( ch.is_ascii_alphabetic() );

            let is_black: bool = ch.is_ascii_uppercase();

            let mut piece: u8 = match is_black {
                true  => ChessColor::BLACK as u8,
                false => ChessColor::WHITE as u8,
            };

            piece |= match ch.to_ascii_lowercase() {
                'p' => ChessPiece::PAWN   as u8,
                'r' => ChessPiece::ROOK   as u8,
                'n' => ChessPiece::KNIGHT as u8,
                'b' => ChessPiece::BISHOP as u8,
                'q' => ChessPiece::QUEEN  as u8,
                'k' => ChessPiece::KING   as u8,
                 _  => panic!("Invalid character in FEN string."),
            };

            board_ptr += 1;

            self.pieces[board_ptr] = piece;

        }

        dbg!(&self.pieces);
    }

    fn make_move(&mut self, start: usize, dst: usize) {
        assert!(start != dst);
        assert!(start >= 0 && start < 64);

        let piece = self.pieces[start];
        self.pieces[start] = 0;
        self.pieces[dst] = piece;
    }
}

