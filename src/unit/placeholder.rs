use macroquad::texture::*;
use crate::render::*;
use macroquad::prelude::WHITE;
use crate::unit::Unit;
use crate::unit::Buildable;

const DEFAULT_HEALTH: u32 = 30;

pub struct BasicUnit {
    health: u32,
    texture: Texture2D,
}
impl Unit for BasicUnit {
    fn render(&self, x: f32, y: f32, scaling: DrawTextureParams) {
        draw_texture_ex(&self.texture, x, y, WHITE, scaling);
    }
}
impl Buildable for BasicUnit {
    fn new(texture: Texture2D) -> Self {
        BasicUnit {
            health: DEFAULT_HEALTH,
            texture,
        }
    }
}

