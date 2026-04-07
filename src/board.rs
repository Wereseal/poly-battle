use macroquad::prelude::Texture2D;
use macroquad::texture;
use macroquad::math::vec2;
use crate::tile::*;
use crate::asset_manager;
use crate::entity;

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
        let tile_asset = asset_manager.get_asset(asset_manager::AssetType::Tile);
        Board {
            x, 
            y,
            width,
            height,
            rows,
            columns,
            content: vec![Tile::new(tile_asset); (columns*rows).try_into().unwrap()],
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
    pub fn add_entity(&mut self, x: u32, y: u32, entity: entity::Entity<'assets>) {
        self.content[((x*self.rows)+y) as usize].assign_entity(entity);
    }
}
