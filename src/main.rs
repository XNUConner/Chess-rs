use speedy2d::{Window, Graphics2D};
use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::window::{
    WindowHandler,
    WindowHelper,
    WindowStartupInfo,
};
use speedy2d::dimen::{Vector2, UVec2};
use speedy2d::shape::Rectangle;
use speedy2d::color::Color;

use std::collections::HashMap;

fn main() {
    env_logger::init();

    let (window_width, window_height) = (800, 800);

    let window = Window::new_centered("Chess", (window_height, window_width)).unwrap();
    window.run_loop( ChessWindowHandler::new(window_height, window_width) );
}

struct ChessWindowHandler {
    window_height: u32,
    window_width: u32,
    board: Board,
}

struct Board {
    dimensions_px: u32,
    window_y_offset: u32, // Space in pixels between buffer area ensuring 1:1 aspect ratio and board (y).
    window_x_offset: u32, // Same for x.
    pieces: PiecesController,
}

impl Board {
    fn new(dimensions_px: u32) -> Self {
        Board {
            dimensions_px,
            window_y_offset: 0,
            window_x_offset: 0,
            pieces: PiecesController::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
        }
    }

    fn resize(&mut self, window_height: u32, window_width: u32) {
        self.dimensions_px = if window_height < window_width { window_height } else { window_width };
        self.window_y_offset = (window_height - self.dimensions_px) / 2;
        self.window_x_offset = (window_width  - self.dimensions_px) / 2;
    }

    fn render_board(&self, graphics: &mut Graphics2D) {
        let square_dimensions = self.dimensions_px as f32 / 8.0;
        for y in 0..8 {
            for x in 0..8 {
                let square_color = match (y + x) % 2 {
                    0 => Color::from_int_rgb(160, 170, 185),
                    _ => Color::from_int_rgb(100, 30, 50),
                };

                log::debug!("Drawing {}x{} quad at: y: {}, x: {}", square_dimensions, square_dimensions, y as f32 * square_dimensions, x as f32 * square_dimensions);
                draw_quad_at((y as f32 * square_dimensions ) + self.window_y_offset as f32, (x as f32 * square_dimensions) + self.window_x_offset as f32, square_dimensions, square_dimensions, square_color, graphics);
            }
        }
    }

    fn render_pieces(&mut self, graphics: &mut Graphics2D) {

        if !self.pieces.images_loaded {
            self.pieces.load_images(graphics);
        }

        for y in 0..8 {
            for x in 0..8 {

                if let Some(piece) = self.pieces.get_piece_at(y, x) {
                    log::info!("Rendering piece @ y: {} x: {}", y, x);
                    let square_dimensions = self.dimensions_px as f32 / 8.0;
                    let window_y = (y as f32 * square_dimensions) + self.window_y_offset as f32; 
                    let window_x = (x as f32 * square_dimensions) + self.window_x_offset as f32;


                    let top_left = Vector2::<f32> { y: window_y, x: window_x };
                    let bottom_right = Vector2::<f32> { y: window_y + square_dimensions, x: window_x + square_dimensions };
                    let rect = Rectangle::<f32>::new(top_left, bottom_right);

                    graphics.draw_rectangle_image(rect, self.pieces.get_image_for_piece(piece.name, piece.color) );
                }

            }
        }

    }
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum PieceColor {
    BLACK,
    WHITE,
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
enum PieceName {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    KING,
    QUEEN,
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
struct Piece {
    name: PieceName,
    color: PieceColor,
}

impl Piece {
    fn new(name: PieceName, color: PieceColor) -> Self {
        Piece {
            name,
            color,
        }
    }
}

#[derive(Debug)]
struct PieceInfo {
    fen_notation: char,
    imagepath_white: &'static str,
    imagepath_black: &'static str,
    image_white: Option<ImageHandle>,
    image_black: Option<ImageHandle>,
}

impl PieceInfo {
    fn new(fen_notation: char, imagepath_white: &'static str, imagepath_black: &'static str) -> PieceInfo {
        assert!(fen_notation.is_ascii() && fen_notation.is_alphabetic());
        PieceInfo {
            fen_notation: fen_notation.to_ascii_lowercase(),
            imagepath_white,
            imagepath_black,
            image_white: None,
            image_black: None,
        }
    }
}

struct PiecesController {
    pieces: Vec<Option<Piece>>,
    pieceinfo_map: HashMap<PieceName, PieceInfo>,
    images_loaded: bool,
}

impl PiecesController {
    fn new(fen: &str) -> Self {
        let mut pc = PiecesController {
            pieces: vec![None; 64],

            pieceinfo_map: HashMap::from([
                (PieceName::PAWN,    PieceInfo::new('p', "img/pl.png", "img/pd.png")),
                (PieceName::ROOK,    PieceInfo::new('r', "img/rl.png", "img/rd.png")),
                (PieceName::KNIGHT,  PieceInfo::new('n', "img/nl.png", "img/nd.png")),
                (PieceName::BISHOP,  PieceInfo::new('b', "img/bl.png", "img/bd.png")),
                (PieceName::KING,    PieceInfo::new('k', "img/kl.png", "img/kd.png")),
                (PieceName::QUEEN,   PieceInfo::new('q', "img/ql.png", "img/qd.png")),
            ]),

            images_loaded: false,
        };

        pc.load_fen(fen);

        pc
    }

    fn load_fen(&mut self, fen: &str) {

        let mut board_index = 0;
        for ch in fen.chars() {

            assert!(ch.is_ascii());

            // Skip '/'
            if ch == '/' { continue; };

            // Handle digits (1-8)
            if let Some(digit) = ch.to_digit(/*radix*/ 10) {

                // FEN should not have a digit not between 1 & 8 (inclusive).
                assert!(digit > 0 && digit <= 8);

                self.pieces[board_index] = None;
                board_index += digit as usize;
                continue;

            } 

            let color = match ch.is_uppercase() {
                true  => PieceColor::WHITE,
                false => PieceColor::BLACK,
            };

            let name = match ch {
                'p' | 'P' => { PieceName::PAWN   },
                'n' | 'N' => { PieceName::KNIGHT },
                'k' | 'K' => { PieceName::KING   },
                'q' | 'Q' => { PieceName::QUEEN  },
                'b' | 'B' => { PieceName::BISHOP },
                'r' | 'R' => { PieceName::ROOK   },
                       _  => { panic!("Invalid letter in FEN string! ({})", ch); },
            };

            let piece = Piece::new(name, color);
            self.pieces[board_index] = Some(piece);
            board_index += 1;

        }

    }

    fn get_piece_at(&self, y: usize, x: usize) -> Option<Piece> {
        let index = (y * 8) + x;
        self.pieces[index]
    }

    fn load_images(&mut self, graphics: &mut Graphics2D) {
        if self.images_loaded { return; };

        for info in &mut self.pieceinfo_map.values_mut() {
            let white_imagehandle = graphics.create_image_from_file_path(None, ImageSmoothingMode::NearestNeighbor, info.imagepath_white).expect("Could not load an image for a white piece.");
            let black_imagehandle = graphics.create_image_from_file_path(None, ImageSmoothingMode::NearestNeighbor, info.imagepath_black).expect("Could not load an image for a black piece.");
            
            info.image_white = Some(white_imagehandle);
            info.image_black = Some(black_imagehandle);

        }
        self.images_loaded = true;
    }

    fn get_image_for_piece(&self, name: PieceName, color: PieceColor) -> &ImageHandle {
        match color {
            PieceColor::WHITE => &self.pieceinfo_map.get(&name).unwrap().image_white.as_ref().unwrap(),
            PieceColor::BLACK => &self.pieceinfo_map.get(&name).unwrap().image_black.as_ref().unwrap(),
        }
    }
}

impl ChessWindowHandler {
    pub fn new(window_height: u32, window_width: u32) -> Self {

        let board_dimensions_px = if window_height < window_width { window_height } else { window_width };

        ChessWindowHandler {
            window_height,
            window_width,
            board: Board::new(board_dimensions_px),
        }

    }
}

impl WindowHandler for ChessWindowHandler {
    fn on_start(&mut self, _helper: &mut WindowHelper, info: WindowStartupInfo) {
        log::info!("on_start() | info: {:?}", info);
        self.window_height = info.viewport_size_pixels().y;
        self.window_width = info.viewport_size_pixels().x;

        self.board.resize(self.window_height, self.window_width);
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper, size_pixels: UVec2) {
        log::info!("on_resize() | size_pixels: {:?}", size_pixels);
        self.window_height = size_pixels.y;
        self.window_width  = size_pixels.x;

        self.board.resize(self.window_height, self.window_width);

    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {

        // Clear the screen
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        let offset_y = ((self.window_height - self.board.dimensions_px) / 2) as f32;
        let offset_x = ((self.window_width  - self.board.dimensions_px) / 2) as f32;

        draw_quad_at(offset_y, offset_x, self.board.dimensions_px as f32, self.board.dimensions_px as f32, Color::GREEN, graphics);

        self.board.render_board(graphics);
        self.board.render_pieces(graphics);

    }

}

fn draw_quad_at(y: f32, x: f32, height: f32, width: f32, color: Color, graphics: &mut Graphics2D) {

    let quad = [
        Vector2::new(width + x, height + y),
        Vector2::new(width + x,          y),
        Vector2::new(        x,          y),
        Vector2::new(        x, height + y),
    ];

    graphics.draw_quad(quad, color);
}
