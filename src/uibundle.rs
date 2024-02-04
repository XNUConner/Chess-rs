use speedy2d::shape::URect;
use speedy2d::dimen::{UVec2, Vector2};
use speedy2d::Graphics2D;


use crate::userinputhandler::UserInputHandler;
use crate::renderer::Renderer;
use crate::chess::Chess;

pub struct UIBundle {
    window_rect:   URect,
    gameview_rect: URect,
    renderer:      Renderer,
    input_handler: UserInputHandler,
    loaded_images: bool,
}

impl UIBundle {
    pub fn new(window_width: u32, window_height: u32, gameview_width: u32, gameview_height: u32) -> Self {

        let (window_rect, gameview_rect) = Self::build_rects(window_width, window_height, gameview_width, gameview_height);

        UIBundle {
            window_rect,
            gameview_rect,
            renderer:       Renderer::new(),
            input_handler:  UserInputHandler::new(),
            loaded_images:  false,
        }
    }

    fn build_rects(window_width: u32, window_height: u32, gameview_width: u32, gameview_height: u32) -> (URect, URect) {
   
        let window_rect = {
            // Build main window rectangle
            let window_topleft     = (/* x */ 0, /* y */ 0);
            let window_bottomright = (window_width, window_height);

            URect::from_tuples(window_topleft, window_bottomright)
        };

        // Build gameview rectangle

        let gameview_rect = {
            // Top left
            let game_topleft = {
                let game_topleft_x = (window_width  - gameview_width)  / 2;
                let game_topleft_y = (window_height - gameview_height) / 2;
    
                (game_topleft_x, game_topleft_y)
            };

            // Bottom right
            let game_bottomright = {
                let game_bottomright_x = game_topleft.0 /* x */ + gameview_width;
                let game_bottomright_y = game_topleft.1 /* y */ + gameview_height;
    
                // topleft asserts are unecessary due to the constraints of u32 having to be >= 0
                assert!(game_bottomright_x <= window_width);
                assert!(game_bottomright_y <= window_height);
    
                (game_bottomright_x, game_bottomright_y)
            };

            URect::from_tuples(game_topleft, game_bottomright)
        };

        // Return rectangles as tuple
        (window_rect, gameview_rect)
    }

    pub fn resize_window(&mut self, window_dimensions: &UVec2) {
        let buffer_size_x = self.gameview_rect.top_left().x;
        let buffer_size_y = self.gameview_rect.top_left().y;

        let window_width    = window_dimensions.x;
        let window_height   = window_dimensions.y;
        let gameview_width  = window_width  - (buffer_size_x * 2);
        let gameview_height = window_height - (buffer_size_y * 2);

        (self.window_rect, self.gameview_rect) = Self::build_rects(window_width, window_height, gameview_width, gameview_height);
    
    }

    pub fn set_hovered_square(&mut self, pos: &Vector2<f32>) {
        self.input_handler.set_hovered_square(&self.window_rect, &self.gameview_rect, pos);
    }

    pub fn mouse_clicked(&mut self, chess: &mut Chess) {
        self.input_handler.mouse_clicked(chess);
    }

    pub fn load_images(&mut self, graphics: &mut Graphics2D) {
        if !self.loaded_images {
            self.renderer.load_images(graphics);
            self.loaded_images = true;
        }
    }

    pub fn draw_chessboard(&self, chess: &Chess, graphics: &mut Graphics2D) {
        self.renderer.draw_chessboard(&self.gameview_rect, chess, graphics);
    }

    pub fn draw_hovered_square(&self, hovered_square: usize, graphics: &mut Graphics2D) {
        Renderer::draw_hovered_square(&self.gameview_rect, hovered_square, graphics);
    }

    pub fn draw_selected_piece_square(&self, selected_piece_square: usize, graphics: &mut Graphics2D) {
        Renderer::draw_selected_piece_square(&self.gameview_rect, selected_piece_square, graphics);
    }

    pub fn get_hovered_square(&self) -> Option<usize> {
        self.input_handler.get_hovered_square()
    }

    pub fn get_selected_piece_square(&self) -> Option<usize> {
        self.input_handler.get_selected_piece_square()
    }

}


