mod window;
mod asset_manager;
mod tile;
mod unit;
mod render;
mod board;


use macroquad::prelude::*;
use macroquad::texture::{self, set_default_filter_mode};
use crate::window::*;
use crate::asset_manager::*;
use crate::unit::placeholder;


#[macroquad::main("poly-battle")]
async fn main() {
    set_default_filter_mode(texture::FilterMode::Nearest);
    let asset_manager = AssetManager::start();
    let mut board = board::Board::new(0.0, 0.0, 1200, 1200, 5, 5, &asset_manager);
    board.add_unit::<placeholder::BasicUnit>(0, 0, &asset_manager.base);
    board.add_unit::<placeholder::BasicUnit>(1, 0, &asset_manager.defender);
    board.add_unit::<placeholder::BasicUnit>(2, 0, &asset_manager.long_range);
    board.add_unit::<placeholder::BasicUnit>(3, 0, &asset_manager.mid_range);
    board.add_unit::<placeholder::BasicUnit>(4, 0, &asset_manager.short_range);
    let white = Color::new(1.0, 1.0, 1.0, 1.0);
    let window = Window::new(1200.0, 1200.0, white);
    loop {
        window.clear();
        board.render();
        next_frame().await
    }
}
