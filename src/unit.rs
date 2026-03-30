pub mod placeholder;

use macroquad::prelude::Texture2D;
use macroquad::prelude::DrawTextureParams;


pub trait Unit {
    fn render(&self, x: f32, y: f32, scaling: DrawTextureParams);
}
pub trait Buildable {
    fn new(texture: &Texture2D) -> Self;
}
