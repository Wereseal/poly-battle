mod window;
mod asset_manager;
mod tile;
mod entity;
mod render;
mod board;


use macroquad::prelude::*;
use macroquad::input::{self, is_quit_requested, is_mouse_button_down};
use macroquad::texture::{self, set_default_filter_mode};
use crate::window::*;


#[macroquad::main("poly-battle")]
async fn main() {
    set_default_filter_mode(texture::FilterMode::Nearest);
    let asset_manager = asset_manager::AssetManager::start();
    let mut board = board::Board::new(0.0, 0.0, 1200, 1200, 10, 10, &asset_manager);
    board.add_entity(0, 0, entity::Entity::new_scout(asset_manager.get_asset(asset_manager::AssetType::Scout{ team: entity::Team::Blue }))); 
    let white = Color::new(1.0, 1.0, 1.0, 1.0);
    let window = Window::new(1200.0, 1200.0, white);
    while !is_quit_requested() && !is_key_down(input::KeyCode::Escape) {
        window.clear();
        board.render();
        next_frame().await;
    }
}
