use speedy2d::{Window, Graphics2D};
use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::window::MouseButton;
use speedy2d::window::{
    WindowHandler,
    WindowHelper,
    WindowStartupInfo,
};
use speedy2d::dimen::{Vector2, UVec2, Vec2};
use speedy2d::shape::{Rectangle, URect};
use speedy2d::color::Color;

use std::collections::HashMap;

const GAME_VIEW_HEIGHT_PX: u32 = 700;
const GAME_VIEW_WIDTH_PX: u32 = 700;

fn main() {

    env_logger::init();

    let (window_width, window_height) = (800, 800);

    let window = Window::new_centered("Chess", (window_height, window_width)).unwrap();

    window.run_loop( GameWindowHandler::new() );

}

struct GameWindowHandler {
    game: Chess,
    renderer: Option<Renderer>,
    selected_square: Option<usize>,
    hovered_square:  Option<usize>,
}

impl GameWindowHandler {
    fn new() -> Self {
        GameWindowHandler {
            game: Chess::new(),
            renderer: None,
            selected_square: None,
            hovered_square: None,
        }
    }
}

impl WindowHandler for GameWindowHandler {
    fn on_start(&mut self, helper: &mut WindowHelper, info: WindowStartupInfo) {
        log::info!("Got on_start callback: {:?}", info);
        let window_width = info.viewport_size_pixels().x;
        let window_height = info.viewport_size_pixels().y;
        self.renderer = Some(Renderer::new( window_width, window_height ));

        self.game.load_fen("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8");
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper, size_pixels: UVec2) {
        log::info!("Got on_resize callback: {:?}", size_pixels);
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {

        // Load piece images if not already done
        if self.renderer.as_ref().unwrap().piece_images.is_none() {
            self.renderer.as_mut().unwrap().load_images(graphics);
        }

        // Clear the screen
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.renderer.as_mut().unwrap().render_board(&self.game.pieces, graphics);
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2) {
        self.hovered_square = self.renderer.as_ref().unwrap().get_board_position(&position);

        helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton)
    {
        log::info!("Got on_mouse_button_down callback: {:?}", button);

        if button == MouseButton::Left && self.hovered_square.is_some() && self.selected_square.is_some() {
            let piece_to_move = self.game.pieces[self.selected_square.unwrap()];
            self.game.pieces[self.selected_square.unwrap()] = 0;
            self.game.pieces[self.hovered_square.unwrap()] = piece_to_move;

            self.selected_square = None;
        }

        if button == MouseButton::Left && self.hovered_square.is_some() && self.selected_square.is_none() {
            if self.game.pieces[self.hovered_square.unwrap()] != 0 {
                self.selected_square = self.hovered_square;
            }
        }


        helper.request_redraw();
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper, button: MouseButton)
    {
        log::info!("Got on_mouse_button_up callback: {:?}", button);

        //helper.request_redraw();
    }
}

#[derive(Debug)]
struct Renderer {
    window_rect: URect,
    game_rect: URect,
    piece_images: Option<HashMap<ChessPiece, [ImageHandle; 2]>>,
}

impl Renderer {
    fn new(window_width: u32, window_height: u32) -> Self {


        let game_topleft_y = (window_height - GAME_VIEW_HEIGHT_PX) / 2;
        let game_topleft_x = (window_width  - GAME_VIEW_WIDTH_PX)  / 2;

        let game_topleft = (game_topleft_x, game_topleft_y);

        let game_bottomright_y = game_topleft_y + GAME_VIEW_HEIGHT_PX;
        let game_bottomright_x = game_topleft_x + GAME_VIEW_WIDTH_PX;
        
        let game_bottomright = (game_bottomright_x, game_bottomright_y);

        // topleft asserts are unecessary due to the constraints of u32 having to be >= 0
        assert!(game_bottomright_y <= window_height);
        assert!(game_bottomright_x <= window_width);

        let window_topleft = (/* x */ 0, /* y */ 0);
        let window_bottomright = (window_width, window_height);

        Renderer {
            window_rect: URect::from_tuples( window_topleft, window_bottomright ),
            game_rect:   URect::from_tuples( game_topleft,   game_bottomright   ),
            piece_images: None,
        }
    }

    // Renderer should not be doing this, need some GameInput struct with a &'static mut Chess
    // reference
    fn get_board_position(&self, mouse_position: &Vector2<f32>) -> Option<usize> {
        let out_of_bounds_y = mouse_position.y < self.game_rect.top_left().y as f32  ||  mouse_position.y > self.game_rect.bottom_right().y as f32;
        let out_of_bounds_x = mouse_position.x < self.game_rect.top_left().x as f32  ||  mouse_position.x > self.game_rect.bottom_right().x as f32;

        if out_of_bounds_y || out_of_bounds_x {
            None
        } else {
            let ratio_x = GAME_VIEW_WIDTH_PX  as f32 / 8.0;
            let ratio_y = GAME_VIEW_HEIGHT_PX as f32 / 8.0;

            let x_offset = (self.game_rect.top_left().x - self.window_rect.top_left().x) as f32;
            let y_offset = (self.game_rect.top_left().y - self.window_rect.top_left().y) as f32;

            let board_pos_x = ((mouse_position.x - x_offset) / ratio_x as f32) as u32;
            let board_pos_y = ((mouse_position.y - y_offset) / ratio_y as f32) as u32;

            Some( ((board_pos_y * 8) + board_pos_x) as usize )
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
            ChessPiece::nfl theme songQUEEN  => 'q',
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

        // Calculate both, but only using width, since for now the board is always made to true
        // squares instead of rectangles with differing height-width ratios.
        let square_len = {
            let _game_rect_height = self.game_rect.bottom_right().y - self.game_rect.top_left().y;
            let game_rect_width  = self.game_rect.bottom_right().x - self.game_rect.top_left().x;

            game_rect_width / 8
        };

        let top_left = (self.game_rect.top_left().x, self.game_rect.top_left().y);

        let bottom_right = {
            let x = top_left.0 + square_len;
            let y = top_left.1 + square_len;
            (x, y)
        };

        // Top left square of board
        let top_left_square = URect::from_tuples(top_left, bottom_right);

        for y in 0..8 {

            for x in 0..8 {

                let square = {
                    let y_offset = y * square_len;
                    let x_offset = x * square_len;
                    top_left_square.with_offset( UVec2::new(x_offset, y_offset) )
                };

                let square_color = match (y + x) % 2 {
                    0 => Color::WHITE,
                    _ => Color::from_int_rgb(200, 200, 200),
                };

                // Due to .as_ref() not being implemented for Rectangle<u32> but only for
                // Rectangle<f32>, we must convert square from URect to Rect
                // Which is a bit strange, since how would you draw 0.5 pixels?
                let square_f32 = {
                    let top_left_f32     = ( square.top_left().x as f32, square.top_left().y as f32 );
                    let bottom_right_f32 = ( square.bottom_right().x as f32, square.bottom_right().y as f32 );

                    Rectangle::from_tuples(top_left_f32, bottom_right_f32)
                };

		graphics.draw_rectangle(&square_f32, square_color);

                let piece = {
                    let board_position = ((y * 8) + x) as usize;
                    pieces[board_position]
                };

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

                graphics.draw_rectangle_image(&square_f32, imagehandle);
                
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

            let is_black: bool = ch.is_ascii_lowercase();

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

            self.pieces[board_ptr] = piece;
            board_ptr += 1;

        }

    }

    fn make_move(&mut self, start: usize, dst: usize) {
        assert!(start != dst);
        assert!(start >= 0 && start < 64);

        let piece = self.pieces[start];
        self.pieces[start] = 0;
        self.pieces[dst] = piece;
    }
}

