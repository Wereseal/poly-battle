use macroquad::prelude::Texture2D;

pub struct AssetManager {
    pub tile: Texture2D,
    pub base_blue: Texture2D,
    pub sniper_blue: Texture2D,
    pub scout_blue: Texture2D,
    pub tank_blue: Texture2D,
    pub defender_blue: Texture2D,
    pub base_red: Texture2D,
    pub sniper_red: Texture2D,
    pub scout_red: Texture2D,
    pub tank_red: Texture2D,
    pub defender_red: Texture2D,
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
}
