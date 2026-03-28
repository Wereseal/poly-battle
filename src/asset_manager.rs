use macroquad::prelude::Texture2D;

pub struct AssetManager {
    pub tile: Texture2D,
    pub long_range: Texture2D,
    pub short_range: Texture2D,
    pub mid_range: Texture2D,
    pub base: Texture2D,
    pub defender: Texture2D,
}
impl AssetManager {
    pub fn start() -> Self {
        AssetManager {
            tile: Texture2D::from_file_with_format(include_bytes!("../assets/tile3.png"), None),
            long_range: Texture2D::from_file_with_format(include_bytes!("../assets/long_range.png"), None),
            short_range: Texture2D::from_file_with_format(include_bytes!("../assets/short_range.png"), None),
            mid_range: Texture2D::from_file_with_format(include_bytes!("../assets/mid_range.png"), None),
            base: Texture2D::from_file_with_format(include_bytes!("../assets/base.png"), None),
            defender: Texture2D::from_file_with_format(include_bytes!("../assets/defender.png"), None),
        }
    }
}
