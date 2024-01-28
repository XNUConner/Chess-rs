use speedy2d::Graphics2D;
use speedy2d::window::{
    WindowHandler,
    WindowHelper,
    WindowStartupInfo,
    MouseButton,
};
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::color::Color;

use crate::uibundle::UIBundle;
use crate::chess::Chess;

pub struct GameWindowHandler {
    ui:    UIBundle,
    chess: Chess,
}

impl GameWindowHandler {
    pub fn new(ui: UIBundle, chess: Chess) -> Self {
        GameWindowHandler {
            ui,
            chess,
        }
    }

}

impl WindowHandler for GameWindowHandler {
    fn on_start(&mut self, helper: &mut WindowHelper, info: WindowStartupInfo) {
        //log::info!("Got on_start callback: {:?}", info);
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper, size_pixels: UVec2) {
        //log::info!("Got on_resize callback: {:?}", size_pixels);
        self.ui.resize_window(&size_pixels);
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {

        // Load piece images if not already done
        // Unfortunately this cannot be done outside of the on_draw() callback due to a dependency on the graphics variable.
        self.ui.load_images(graphics);

        // Sets the background color
        graphics.clear_screen( Color::from_int_rgb(30, 16, 16) );

        // Render the chess board
        self.ui.render_board(&self.chess.board(), graphics);
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2) {
        self.ui.set_hovered_square(&position);
        //log::info!("{:?}", position);
        //log::info!("{:?}", self.ui.get_hovered_square());
        helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        //log::info!("{:?}", button);
        self.ui.mouse_clicked(&mut self.chess);
        helper.request_redraw();
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        //log::info!("{:?}", button);
        helper.request_redraw();
    }
}