
use std::collections::HashMap;

use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::shape::{Rectangle, URect};
use speedy2d::dimen::UVec2;
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

    pub fn render_board(&self, gameview_rect: &URect, board: &[u8], graphics: &mut Graphics2D) {

        // Prepare the dimensions and coordinates of the board square to be drawn

        // Calculate both, but only using width, since for now the board is always made to true
        // squares instead of rectangles with differing height-width ratios.
        let square_len = {
            let _gameview_rect_height = gameview_rect.bottom_right().y - gameview_rect.top_left().y;
            let gameview_rect_width   = gameview_rect.bottom_right().x - gameview_rect.top_left().x;

            gameview_rect_width / 8
        };

        let top_left = (gameview_rect.top_left().x, gameview_rect.top_left().y);

        let bottom_right = {
            let x = top_left.0 /* x */ + square_len;
            let y = top_left.1 /* y */ + square_len;
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
                    0 => Color::from_int_rgb(253, 245, 245), /* light */
                    _ => Color::from_int_rgb(36, 78, 36), /* dark */
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
                    board[board_position]
                };

                // Skip if no piece at that position
                if piece == 0 { continue; }

                // extract color
                let color = Chess::get_color_for_piece(piece);

                // Piece contains both the piece color (MSB and 2nd-MSB) and name (1 of 6 LSB), this code extracts the name with XOR
                let name = PieceName::try_from(piece ^ (color as u8)).unwrap();

                let imagehandle = match color {
                    PieceColor::WHITE => &self.piece_images.as_ref().unwrap().get(&name).unwrap()[0],
                    PieceColor::BLACK => &self.piece_images.as_ref().unwrap().get(&name).unwrap()[1],
                };

                graphics.draw_rectangle_image(&square_f32, imagehandle);
                
            }
        }
        

    }
}