use macroquad::prelude::Texture2D;
use macroquad::texture;
use macroquad::math::vec2;
use crate::tile::*;
use crate::asset_manager;
use crate::unit;
use crate::structure;

pub struct Board<'assets> {
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    rows: u32,
    columns: u32,
    content: Vec<Tile<'assets>>,
    scaling: texture::DrawTextureParams,
}
impl<'assets> Board<'assets> {
    pub fn new(x: f32, y: f32, width: u32, height: u32, columns: u32, rows: u32, asset_manager: &'assets asset_manager::AssetManager) -> Self {
        Board {
            x, 
            y,
            width,
            height,
            rows,
            columns,
            content: vec![Tile::new(&asset_manager.tile); (columns*rows).try_into().unwrap()],
            scaling: texture::DrawTextureParams {
                dest_size: Some(vec2((width as f32)/(columns as f32), (height as f32)/(rows as f32))),
                ..Default::default()
            },
        }
    }
    pub fn render(&self) {
        let tile_width: f32 = (self.width as f32)/(self.columns as f32);
        let tile_height: f32 = (self.height as f32)/(self.rows as f32);
        for i in 0..self.columns {
            for j in 0..self.rows {
                self.content[((i*self.rows)+j) as usize].render((tile_width*(i as f32))+self.x, (tile_height*(j as f32))+self.y,  &self.scaling);
            }
        }
    }
    pub fn add_structure(&mut self, x: u32, y: u32, structure_type: structure::StructureType, texture: &'assets Texture2D) {
        self.content[((x*self.rows)+y) as usize].add_structure(structure_type, texture);
    }
    pub fn add_unit(&mut self, x: u32, y: u32, unit_type: unit::UnitType, texture: &'assets Texture2D) {
        self.content[((x*self.rows)+y) as usize].assign_unit(unit_type, texture);
    }
}
