use macroquad::texture::*;
use macroquad::texture;
use crate::unit;

pub struct Tile {
    texture: Texture2D,
    occupant: Option<Box<dyn unit::Unit>>,
}
impl Tile {
    pub fn new(texture: Texture2D) -> Self {
        Tile {
            texture,
            occupant: None,
        }
    }
    pub fn render(&self, x: f32, y: f32, scaling: &texture::DrawTextureParams) {
        draw_texture_ex(&self.texture, x, y, macroquad::prelude::WHITE, scaling.clone());
        if let Some(oc) = &self.occupant {
            oc.render(x, y, scaling.clone());
        }
    }
    pub fn assign_unit<T: unit::Buildable + unit::Unit + 'static>(&mut self, texture: Texture2D) {
        match &self.occupant {
            Some(_) => panic!("Whoops tried to assign a unit to a full tile"),
            None => self.occupant = Some(Box::new(T::new(texture))),
        }
    }
}
impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile {
            texture: self.texture.clone(),
            occupant: None,
        }
    }
}
