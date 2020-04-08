use ggez::{
    timer,
    event::{EventHandler, KeyCode, KeyMods},
    graphics::{self, Image, Font},
    audio::{Source, SoundSource},
    Context, GameResult,
};
use std::{
    path::Path,
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher,
    time::SystemTime,
};

pub mod game;
mod menu;
pub mod actor;

use crate::settings::Settings;
use game::GameState;

pub enum StateID {
    Game,
    Menu,
}

pub trait State {
    fn update(&mut self, ctx: &mut Context, settings: &Settings) -> GameResult<StateID>;
    fn draw(&mut self, ctx: &mut Context, settings: &Settings, res: &Resources) -> GameResult<()>;

    fn key_down_event(&mut self, _ctx: &mut Context, settings: &Settings, keycode: KeyCode, _keymods: KeyMods, repeat: bool) -> StateID;
}

pub struct Resources {
    _sound: Source,
    pub tileset: Image,
    pub background: Image,
    pub popup: Image,
    pub font: Font,
}

impl Resources {
    fn new(ctx: &mut Context, settings: &Settings) -> GameResult<Resources> {
        let mut _sound = Source::new(ctx, Path::new(&settings.sound.file))?;
        if settings.sound.enabled {
            _sound.set_repeat(true);
            _sound.set_volume(settings.sound.volume);
            _sound.play()?;
        }

        let tileset = Image::new(ctx, Path::new(&settings.tile.file))?;

	let background_settings = settings.background();
        let background = Image::new(ctx, Path::new(&background_settings.file))?;
        let popup = Image::new(ctx, Path::new(&background_settings.popup.file))?;
        let font = Font::new(ctx, Path::new(&settings.font.file))?;

        let res = Resources {
            _sound,
            tileset,
            background,
            popup,
            font,
        };

        Ok(res)
    }
}

pub struct StateHandler {
    settings: Settings,
    res: Resources,

    state: GameState,
}

impl StateHandler {
    pub fn new(ctx: &mut Context, settings: Settings) -> GameResult<StateHandler> {
        let res =  Resources::new(ctx, &settings)?;
        let seed = StateHandler::generate_seed();
        let state = GameState::new(&settings, &res, seed)?;

        let handler = StateHandler {
            settings,
            res,

            state,
        };

        Ok(handler)
    }

    fn generate_seed() -> [u8; 32] {
        let mut seed: [u8; 32] = [0; 32];

        for i in 0..4 {
            let mut hasher = DefaultHasher::new();
            SystemTime::now().hash(&mut hasher);
            let hash = hasher.finish();

            for j in 0..8 {
                seed[i * 8 + j] = ((hash >> (j << 3)) & 0xFF) as u8;
            }
        }

        seed
    }
}

impl EventHandler for StateHandler {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // println!("FPS: {:?} - Ticks: {}", timer::fps(ctx), timer::ticks(ctx));

	self.state.update(ctx, &self.settings)?;

        timer::yield_now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.state.draw(ctx, &self.settings, &self.res)?;

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        self.state.key_down_event(ctx, &self.settings, keycode, keymods, repeat);
    }
}
