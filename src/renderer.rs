
use std::collections::HashMap;

use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::shape::{Rectangle, URect};
use speedy2d::dimen::{Vec2, UVec2};
use speedy2d::color::Color;
use speedy2d::Graphics2D;

use crate::chess::Chess;
use crate::chess::{PieceColor, PieceName};

#[derive(Debug)]
pub struct Renderer {
    piece_images: Option<HashMap<PieceName, [ImageHandle; 2]>>,
}

impl Renderer {
    pub fn new() -> Self {

        Renderer {
            piece_images: None,
        }
    }

    pub fn load_images(&mut self, graphics: &mut Graphics2D) {
        self.piece_images = Some( HashMap::from([
            (PieceName::PAWN,   Self::load_images_for_piece(PieceName::PAWN,   graphics)),
            (PieceName::ROOK,   Self::load_images_for_piece(PieceName::ROOK,   graphics)),
            (PieceName::KNIGHT, Self::load_images_for_piece(PieceName::KNIGHT, graphics)),
            (PieceName::BISHOP, Self::load_images_for_piece(PieceName::BISHOP, graphics)),
            (PieceName::QUEEN,  Self::load_images_for_piece(PieceName::QUEEN,  graphics)),
            (PieceName::KING,   Self::load_images_for_piece(PieceName::KING,   graphics)),
        ]));
    }

    pub fn load_images_for_piece(piece: PieceName, graphics: &mut Graphics2D) -> [ImageHandle; 2] {


        const IMAGE_DIR: &str = "img/";
        const IMAGE_EXT: &str = ".png";

        let piece_char = match piece {
            PieceName::PAWN   => 'p',
            PieceName::ROOK   => 'r',
            PieceName::KNIGHT => 'n',
            PieceName::BISHOP => 'b',
            PieceName::QUEEN  => 'q',
            PieceName::KING   => 'k',
        };

        let light_char = 'l';
        let dark_char  = 'd';
        let path_light = format!("{IMAGE_DIR}{piece_char}{light_char}{IMAGE_EXT}");
        let path_dark  = format!("{IMAGE_DIR}{piece_char}{dark_char}{IMAGE_EXT}");


        [
            graphics.create_image_from_file_path(None, ImageSmoothingMode::NearestNeighbor, path_light.clone()).expect( &format!("Failed to load image for {}", path_light).to_owned()),
            graphics.create_image_from_file_path(None, ImageSmoothingMode::NearestNeighbor,  path_dark.clone()).expect( &format!("Failed to load image for {}", path_dark).to_owned()),
        ]
    } 

    pub fn draw_chessboard(&self, gameview_rect: &URect, chess: &Chess, graphics: &mut Graphics2D) {

        for square in 0..=63 {

            Self::draw_square(gameview_rect, square, graphics);

            if let Some(piece) = chess.get_piece_at_square(square) {
                self.draw_piece(gameview_rect, square, piece, graphics);
            }

        }
    }

    fn draw_piece(&self, gameview_rect: &URect, square: usize, piece: u8, graphics: &mut Graphics2D) {
        let rect = Self::make_rect_for_square(gameview_rect, square);
        let name  = Chess::get_name_for_piece(piece);

        let imagehandle = match Chess::get_color_for_piece(piece) {
            PieceColor::WHITE => &self.piece_images.as_ref().unwrap().get(&name).unwrap()[0],
            PieceColor::BLACK => &self.piece_images.as_ref().unwrap().get(&name).unwrap()[1],
        };

        graphics.draw_rectangle_image(&rect, imagehandle);
    }

    fn draw_square(gameview_rect: &URect, square: usize, graphics: &mut Graphics2D) {
        let rect = Self::make_rect_for_square(gameview_rect, square);

        // Gets us a checkerboard pattern
        let rect_color = match (square + square / 8 % 2 ) % 2 {
            0 => Color::from_int_rgb(253, 245, 245), /* light */
            _ => Color::from_int_rgb(36, 78, 36),    /* dark */
        };

        graphics.draw_rectangle(&rect, rect_color);
    }

    // Only used for hovered square right now
    pub fn draw_hovered_square(gameview_rect: &URect, square: usize, graphics: &mut Graphics2D) {
        let rect = Self::make_rect_for_square(gameview_rect, square);
        let square_color = Color::from_int_rgba(255, 255, 0, 127);
        graphics.draw_rectangle(&rect, square_color);
    }

    pub fn draw_selected_piece_square(gameview_rect: &URect, square: usize, graphics: &mut Graphics2D) {
        let rect = Self::make_rect_for_square(gameview_rect, square);
        let square_color = Color::from_int_rgba(166, 22, 43, 80);
        graphics.draw_rectangle(&rect, square_color);
    }

    fn calc_square_length(gameview_rect: &URect) -> u32 {
        let _gameview_rect_height = gameview_rect.bottom_right().y - gameview_rect.top_left().y;
        let gameview_rect_width   = gameview_rect.bottom_right().x - gameview_rect.top_left().x;

        gameview_rect_width / 8
    }

    fn make_rect_for_square(gameview_rect: &URect, square: usize) -> Rectangle<f32> {
        let rect_len = Self::calc_square_length(gameview_rect);
        let x = gameview_rect.top_left().x as f32 + (square % 8) as f32 * rect_len as f32;
        let y = gameview_rect.top_left().y as f32 + (square / 8) as f32 * rect_len as f32;

        let top_left = Vec2::new(x, y);
        let bottom_right = Vec2::new(x + rect_len as f32, y + rect_len as f32);

        Rectangle::new(top_left, bottom_right)

    }
}