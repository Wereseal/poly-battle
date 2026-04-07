use macroquad::prelude::Texture2D;
use macroquad::prelude::DrawTextureParams;
use macroquad::prelude::draw_texture_ex;
use macroquad::texture;

#[derive(Clone, Copy)]
pub enum StructureType{
    base,
}
pub struct Structure<'assets>{
    health: u32, 
    damage: Option<u32>, 
    texture: &'assets Texture2D,
    structure_type: StructureType,
}
impl Structure<'_> {
    pub fn new<'assets>(structure_type: StructureType, texture: &'assets Texture2D) -> Structure<'assets> {
        match structure_type {
            StructureType::base => Structure{ health: 50, damage: None, texture, structure_type, },
        }
    }
    pub fn render(&self, x: f32, y: f32, scaling: &texture::DrawTextureParams) {
        draw_texture_ex(self.texture, x, y, macroquad::prelude::WHITE, scaling.clone());
    }
}
impl Clone for Structure<'_> {
    fn clone(&self) -> Self {
        Structure {
            health: self.health,
            damage: self.damage,
            texture: self.texture,
            structure_type: self.structure_type,
        }
    }
}
