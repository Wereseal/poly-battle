use macroquad::prelude::Texture2D;
use macroquad::prelude::DrawTextureParams;
use macroquad::prelude::draw_texture_ex;
use macroquad::texture;

#[derive(PartialEq)]
pub enum Team {
    Red,
    Blue,
}

#[derive(Clone, Copy)]
pub enum Entity<'assets> {
    Scout { health: u32, damage: u32, range: u32, speed: u32, cost: u32, texture: &'assets Texture2D, },
    Tank { health: u32, damage: u32, range: u32, speed: u32, cost: u32, texture: &'assets Texture2D, },
    Sniper { health: u32, damage: u32, range: u32, speed: u32, cost: u32, texture: &'assets Texture2D, },
    Defender { health: u32, damage: u32, range: u32, speed: u32, cost: u32, texture: &'assets Texture2D, },
    Base { health: u32, texture: &'assets Texture2D, },
    
}
impl Entity<'_> {
    pub fn new_scout<'assets>(texture: &'assets Texture2D) -> Entity<'assets> {
        Entity::Scout {
            health: 5, 
            damage: 2,
            range: 1,
            speed: 10,
            cost: 5,
            texture,
        }
    }
    pub fn new_tank<'assets>(texture: &'assets Texture2D,) -> Entity<'assets> {
        Entity::Tank {
            health: 20,
            damage: 5,
            range: 3,
            speed: 5,
            cost: 30,
            texture,
        }
    }
    pub fn new_sniper<'assets>(texture: &'assets Texture2D,) -> Entity<'assets> {
        Entity::Sniper {
            health: 10,
            damage: 10,
            range: 10,
            speed: 1,
            cost: 20,
            texture,
        }
    }
    pub fn new_defender<'assets>(texture: &'assets Texture2D,) -> Entity<'assets> {
        Entity::Defender {
            health: 15,
            damage: 10,
            range: 1,
            speed: 1,
            cost: 10,
            texture,
        }
    }
    pub fn new_base<'assets>(texture: &'assets Texture2D,) -> Entity<'assets> {
        Entity::Base {
            health: 50,
            texture,
        }
    }
    pub fn render(&self, x: f32, y: f32, scaling: &texture::DrawTextureParams) {
        match self {
            Entity::Scout{health: _, damage: _, range: _, speed: _, cost: _, texture} => draw_texture_ex(texture, x, y, macroquad::prelude::WHITE, scaling.clone()),
            Entity::Tank{health: _, damage: _, range: _, speed: _, cost: _, texture} => draw_texture_ex(texture, x, y, macroquad::prelude::WHITE, scaling.clone()),
            Entity::Sniper{health: _, damage: _, range: _, speed: _, cost: _, texture} => draw_texture_ex(texture, x, y, macroquad::prelude::WHITE, scaling.clone()),
            Entity::Defender{health: _, damage: _, range: _, speed: _, cost: _, texture} => draw_texture_ex(texture, x, y, macroquad::prelude::WHITE, scaling.clone()),
            Entity::Base{health: _, texture} => draw_texture_ex(texture, x, y, macroquad::prelude::WHITE, scaling.clone()),
        }
    }
}


