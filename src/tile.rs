use macroquad::texture::*;
use macroquad::texture;
use crate::unit;
use crate::structure;

pub struct Tile<'assets> {
    occupant: Option<unit::Unit<'assets>>,
    building: Option<structure::Structure<'assets>>,
    texture: &'assets Texture2D,
}

impl<'assets> Tile<'assets> {
    pub fn new(texture: &'assets Texture2D) -> Self {
        Tile {
            texture,
            building: None,
            occupant: None,
        }
    }
    pub fn render(&self, x: f32, y: f32, scaling: &texture::DrawTextureParams) {
        draw_texture_ex(self.texture, x, y, macroquad::prelude::WHITE, scaling.clone());
        if let Some(oc) = &self.building {
            oc.render(x, y, scaling);
        }
        if let Some(oc) = &self.occupant {
            oc.render(x, y, scaling);
        }
    }
    pub fn add_structure(&mut self, structure_type: structure::StructureType, texture: &'assets Texture2D) {
        match &self.building {
            Some(_) => panic!("Whoops tried to assign a building to a full tile"),
            None => self.building = Some(structure::Structure::new(structure_type, texture)),
        }
    }
    pub fn assign_unit(&mut self, unit_type: unit::UnitType, texture: &'assets Texture2D) {
        match &self.occupant {
            Some(_) => panic!("Whoops tried to assign a unit to a full tile"),
            None => self.occupant = Some(unit::Unit::new(unit_type, texture)),
        }
    }
}
impl Clone for Tile<'_> {
    fn clone(&self) -> Self {
        Tile {
            //occupant: self.occupant.clone(),
            occupant: None,
            building: None,
            texture: self.texture,
        }
    }
}
