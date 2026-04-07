use macroquad::prelude::Texture2D;
use macroquad::prelude::DrawTextureParams;
use macroquad::prelude::draw_texture_ex;
use macroquad::texture;

#[derive(Clone, Copy)]
pub enum UnitType {
    Scout,
    Tank,
    Sniper,
    Defender,
}
pub struct Unit<'assets>{
    health: u32, 
    damage: u32, 
    texture: &'assets Texture2D,
    unit_type: UnitType,
}
impl Unit<'_> {
    pub fn new<'assets>(unit_type: UnitType, texture: &'assets Texture2D) -> Unit<'assets> {
        match unit_type {
            UnitType::Scout => Unit{ health: 5, damage: 2, texture, unit_type, },
            UnitType::Tank => Unit{ health: 20, damage: 5, texture, unit_type, },
            UnitType::Sniper => Unit{ health: 10, damage: 10, texture, unit_type, },
            UnitType::Defender => Unit{ health: 10, damage: 10, texture, unit_type, },
        }
    }
    pub fn render(&self, x: f32, y: f32, scaling: &texture::DrawTextureParams) {
        draw_texture_ex(self.texture, x, y, macroquad::prelude::WHITE, scaling.clone());
    }
}
impl Clone for Unit<'_> {
    fn clone(&self) -> Self {
        Unit {
            health: self.health,
            damage: self.damage,
            texture: self.texture,
            unit_type: self.unit_type,
        }
    }
}
