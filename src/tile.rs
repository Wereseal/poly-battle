use macroquad::texture::*;
use macroquad::texture;
use crate::entity;

pub struct Tile<'assets> {
    occupant: Option<entity::Entity<'assets>>,
    texture: &'assets Texture2D,
}

impl<'assets> Tile<'assets> {
    pub fn new(texture: &'assets Texture2D) -> Self {
        Tile {
            texture,
            occupant: None,
        }
    }
    pub fn render(&self, x: f32, y: f32, scaling: &texture::DrawTextureParams) {
        draw_texture_ex(self.texture, x, y, macroquad::prelude::WHITE, scaling.clone());
        if let Some(oc) = &self.occupant {
            oc.render(x, y, scaling);
        }
    }
    pub fn assign_entity(&mut self, entity: entity::Entity<'assets>) {
        match &self.occupant {
            Some(_) => panic!("Whoops tried to assign a entity to a full tile"),
            None => self.occupant = Some(entity),
        }
    }
}
impl Clone for Tile<'_> {
    fn clone(&self) -> Self {
        Tile {
            occupant: None,
            texture: self.texture,
        }
    }
}
