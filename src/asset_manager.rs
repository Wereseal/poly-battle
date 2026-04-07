use macroquad::prelude::Texture2D;
use crate::entity;

pub enum AssetType {
    Tile,
    Base{team: entity::Team},
    Sniper{team: entity::Team},
    Scout{team: entity::Team},
    Tank{team: entity::Team},
    Defender{team: entity::Team},
}
pub struct AssetManager {
    tile: Texture2D,
    base_blue: Texture2D,
    sniper_blue: Texture2D,
    scout_blue: Texture2D,
    tank_blue: Texture2D,
    defender_blue: Texture2D,
    base_red: Texture2D,
    sniper_red: Texture2D,
    scout_red: Texture2D,
    tank_red: Texture2D,
    defender_red: Texture2D,
}
impl AssetManager {
    pub fn start() -> Self {
        AssetManager {
            tile: Texture2D::from_file_with_format(include_bytes!("../assets/tile.png"), None),
            base_blue: Texture2D::from_file_with_format(include_bytes!("../assets/base_blue.png"), None),
            sniper_blue: Texture2D::from_file_with_format(include_bytes!("../assets/long_range_blue.png"), None),
            scout_blue: Texture2D::from_file_with_format(include_bytes!("../assets/short_range_blue.png"), None),
            tank_blue: Texture2D::from_file_with_format(include_bytes!("../assets/mid_range_blue.png"), None),
            defender_blue: Texture2D::from_file_with_format(include_bytes!("../assets/defender_blue.png"), None),
            base_red: Texture2D::from_file_with_format(include_bytes!("../assets/base_red.png"), None),
            sniper_red: Texture2D::from_file_with_format(include_bytes!("../assets/long_range_red.png"), None),
            scout_red: Texture2D::from_file_with_format(include_bytes!("../assets/short_range_red.png"), None),
            tank_red: Texture2D::from_file_with_format(include_bytes!("../assets/mid_range_red.png"), None),
            defender_red: Texture2D::from_file_with_format(include_bytes!("../assets/defender_red.png"), None),
        }
    }
    pub fn get_asset(&self, asset_type: AssetType) -> &Texture2D{
        match asset_type {
            AssetType::Tile => &self.tile,
            AssetType::Base{ team } => if team == entity::Team::Blue { &self.base_blue } else { &self.base_red },
            AssetType::Sniper{ team } => if team == entity::Team::Blue { &self.sniper_blue } else { &self.sniper_red },
            AssetType::Scout{ team } => if team == entity::Team::Blue { &self.scout_blue } else { &self.scout_blue },
            AssetType::Tank{ team } => if team == entity::Team::Blue { &self.tank_blue } else { &self.tank_red },
            AssetType::Defender{ team } => if team == entity::Team::Blue { &self.defender_blue } else { &self.defender_red },
        }
    }
}
