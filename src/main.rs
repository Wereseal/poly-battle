mod window;
mod asset_manager;
mod tile;
mod unit;
mod structure;
mod render;
mod board;


use macroquad::prelude::*;
use macroquad::input::{self, is_quit_requested, is_mouse_button_down};
use macroquad::texture::{self, set_default_filter_mode};
use crate::window::*;
//use crate::unit;
use crate::asset_manager::*;


#[macroquad::main("poly-battle")]
async fn main() {
    set_default_filter_mode(texture::FilterMode::Nearest);
    let asset_manager = AssetManager::start();
    let mut board = board::Board::new(0.0, 0.0, 1200, 1200, 4, 4, &asset_manager);
    board.add_unit(0, 0, unit::UnitType::Scout, &asset_manager.scout_blue);
    board.add_unit(1, 0, unit::UnitType::Defender, &asset_manager.defender_blue);
    board.add_unit(2, 0, unit::UnitType::Sniper, &asset_manager.sniper_blue);
    board.add_unit(3, 0, unit::UnitType::Tank, &asset_manager.tank_blue);
    board.add_structure(0, 1, structure::StructureType::base, &asset_manager.base_blue);
    board.add_structure(1, 1, structure::StructureType::base, &asset_manager.base_blue);
    board.add_structure(2, 1, structure::StructureType::base, &asset_manager.base_blue);
    board.add_structure(3, 1, structure::StructureType::base, &asset_manager.base_blue);
    board.add_unit(0, 1, unit::UnitType::Scout, &asset_manager.scout_blue);
    board.add_unit(1, 1, unit::UnitType::Defender, &asset_manager.defender_blue);
    board.add_unit(2, 1, unit::UnitType::Sniper, &asset_manager.sniper_blue);
    board.add_unit(3, 1, unit::UnitType::Tank, &asset_manager.tank_blue);
    board.add_unit(0, 2, unit::UnitType::Scout, &asset_manager.scout_red);
    board.add_unit(1, 2, unit::UnitType::Defender, &asset_manager.defender_red);
    board.add_unit(2, 2, unit::UnitType::Sniper, &asset_manager.sniper_red);
    board.add_unit(3, 2, unit::UnitType::Tank, &asset_manager.tank_red);
    board.add_structure(0, 3, structure::StructureType::base, &asset_manager.base_red);
    board.add_structure(1, 3, structure::StructureType::base, &asset_manager.base_red);
    board.add_structure(2, 3, structure::StructureType::base, &asset_manager.base_red);
    board.add_structure(3, 3, structure::StructureType::base, &asset_manager.base_red);
    board.add_unit(0, 3, unit::UnitType::Scout, &asset_manager.scout_red);
    board.add_unit(1, 3, unit::UnitType::Defender, &asset_manager.defender_red);
    board.add_unit(2, 3, unit::UnitType::Sniper, &asset_manager.sniper_red);
    board.add_unit(3, 3, unit::UnitType::Tank, &asset_manager.tank_red);
    let white = Color::new(1.0, 1.0, 1.0, 1.0);
    let window = Window::new(1200.0, 1200.0, white);
    while !is_quit_requested() && !is_key_down(input::KeyCode::Escape) {
        window.clear();
        board.render();
        next_frame().await;
    }
}
