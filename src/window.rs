use macroquad::color::Color;
use macroquad::window::*;

pub struct Window {
    height: f32,
    width: f32,
    background_color: Color,
}
impl window {
    pub fn new(height: f32, width: f32, background_color: Color) -> Self {
        Window {
            height,
            width,
            background_color,
        }
    }
    pub fn resize(&mut self, height: f32, width: f32) {
        self.height = height;
        self.width = width;
    }
    pub fn recolor(&mut self, color: Color) {
        self.background_color = color;
    }
    pub fn clear(&self) {
        if self.height != screen_height() || self.width != screen_width() {
            request_new_screen_size(self.width, self.height);
        }
        clear_background(background_color);
    }
}

