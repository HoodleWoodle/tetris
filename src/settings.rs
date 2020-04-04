use serde::Deserialize;
use serde_json::Result;
use std::io::Read;
use crate::random::RandomGeneratorType;

pub const MAP_WIDTH: usize = 10;
pub const MAP_HEIGHT: usize = 22;
pub const MAP_TILE_COUNT: usize = MAP_WIDTH * MAP_HEIGHT;

#[derive(Deserialize)]
pub struct Bounds {
	pub x: f32,
	pub y: f32,
	pub w: f32,
	pub h: f32,
}

#[derive(Deserialize)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

#[derive(Deserialize)]
pub struct SoundSettings {
	pub file: String,
	pub enabled: bool,
	pub volume: f32,
}

#[derive(Deserialize)]
pub struct BackgroundSettings {
	pub file: String,
	pub w: f32,
	pub h: f32,
    pub popup: PopupSettings,
    pub gray_color: [f32; 4],
}

#[derive(Deserialize)]
pub struct PopupSettings {
	pub file: String,
	pub bounds: Bounds,
}

#[derive(Deserialize)]
pub struct TileSettings {
	pub file: String,
	pub size: f32,
}

#[derive(Deserialize)]
pub struct FontSettings {
	pub file: String,
	pub next_text_y_offset: f32,
	pub size_default: f32,
	pub size_player: f32,
	pub size_popup: f32,
}

#[derive(Deserialize)]
pub struct Settings {
    pub random_generator: RandomGeneratorType,
    pub start_level: usize,
    pub wall_kicks_enabled: bool,
    pub hard_drop_enabled: bool,

    pub nickname: String,
    pub connection: String,
    pub multiplayer_enabled: bool,
    
    singleplayer: BackgroundSettings,
    multiplayer: BackgroundSettings,

    pub sound: SoundSettings,
    pub tile: TileSettings,
    pub font: FontSettings,

    pub player_bounds: [Bounds; 2],
    pub score_bounds: [Bounds; 2],
    pub lines_bounds: [Bounds; 2],
    pub level_bounds: [Bounds; 2],
    pub next_bounds: [Bounds; 2],
    pub map_positions: [Point; 2],
}

impl Settings {
    pub fn background(&self) -> &BackgroundSettings {
        if self.multiplayer_enabled {
            &self.multiplayer
        } else {
            &self.singleplayer
        }
    }
}

pub fn load<R: Read>(reader: R) -> Result<Settings> {
    serde_json::from_reader(reader)
}
