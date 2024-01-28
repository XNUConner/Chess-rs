use speedy2d::shape::URect;
use speedy2d::dimen::Vector2;

use crate::chess::Chess;

pub struct UserInputHandler {
    hovered_square:  Option<u8>,
    src_square:      Option<u8>,
    dst_square:      Option<u8>,
}

impl UserInputHandler {
    pub fn new() -> Self {
        UserInputHandler {
            hovered_square: None,
            src_square:     None,
            dst_square:     None,
        }
    }

    pub fn mouse_clicked(&mut self, chess: &mut Chess) {
        if self.hovered_square.is_some() {
            self.square_clicked(chess);
        }
    }

    fn square_clicked(&mut self, chess: &mut Chess) {
        assert!(self.hovered_square.is_some());
        
        if self.src_square.is_some() && self.src_square.unwrap() != self.hovered_square.unwrap() {
            self.dst_square = self.hovered_square;

            // make move
            let src = self.src_square.unwrap();
            let dst = self.dst_square.unwrap();
            assert!(src != dst);

            chess.attempt_move(src, dst);

            self.src_square = None;
            self.dst_square = None;

        } else {
            // Need some sort of method on chess like .is_square_empty(square)
            if chess.board()[self.hovered_square.unwrap() as usize] != 0 {
                self.src_square = self.hovered_square;
            }
        }

    }

    pub fn set_hovered_square(&mut self, window_rect: &URect, gameview_rect: &URect, mouse_position: &Vector2<f32>) {
        let out_of_bounds_y = mouse_position.y < gameview_rect.top_left().y as f32  ||  mouse_position.y > gameview_rect.bottom_right().y as f32;
        let out_of_bounds_x = mouse_position.x < gameview_rect.top_left().x as f32  ||  mouse_position.x > gameview_rect.bottom_right().x as f32;

        if out_of_bounds_y || out_of_bounds_x { 
            self.hovered_square = None;
            return;
         }

        assert!(mouse_position.x >= gameview_rect.top_left().x as f32);
        assert!(mouse_position.x <= gameview_rect.bottom_right().x as f32);
        assert!(mouse_position.y >= gameview_rect.top_left().y as f32);
        assert!(mouse_position.y <= gameview_rect.bottom_right().y as f32);


        let board_pos_x = {

            let ratio_x = {
                let gameview_width  = gameview_rect.bottom_right().x - gameview_rect.top_left().x;
                gameview_width as f32 / 8.0
            };

            let x_offset = (gameview_rect.top_left().x - window_rect.top_left().x) as f32;

            ((mouse_position.x - x_offset) / ratio_x as f32) as u32
        };

        let board_pos_y = {

            let ratio_y = {
                let gameview_height = gameview_rect.bottom_right().y - gameview_rect.top_left().y;
                gameview_height as f32 / 8.0
            };

            let y_offset = (gameview_rect.top_left().y - window_rect.top_left().y) as f32;

            ((mouse_position.y - y_offset) / ratio_y as f32) as u32
        };


        let square = (board_pos_y * 8 + board_pos_x) as u8;
        self.hovered_square = Some( square );

    }

    pub fn get_hovered_square(&self) -> Option<u8> {
        self.hovered_square
    }

}