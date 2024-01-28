use speedy2d::Window;

mod chess;
mod gamewindowhandler;
mod uibundle;
mod renderer;
mod userinputhandler;

use crate::gamewindowhandler::GameWindowHandler;
use crate::uibundle::UIBundle;
use crate::chess::Chess;

const WINDOW_HEIGHT_PX:    u32 = 800;
const WINDOW_WIDTH_PX:     u32 = 800;

const GAMEVIEW_HEIGHT_PX: u32 = 750;
const GAMEVIEW_WIDTH_PX:  u32 = 750;

fn main() {

    env_logger::init();

    let window        = Window::new_centered("Chess", (WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX)).unwrap();
    let ui            = UIBundle::new(WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX, GAMEVIEW_WIDTH_PX, GAMEVIEW_HEIGHT_PX);
    let chess         = Chess::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    window.run_loop( GameWindowHandler::new( ui, chess ) );

}