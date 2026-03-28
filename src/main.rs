mod window;

use crate::window::*;
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let white = color::new(1.0, 1.0, 1.0, 1.0);
    let window = Window::new(2000.0, 1200.0, white);
    loop {
        window.clear();
        next_frame().await
    }
}
