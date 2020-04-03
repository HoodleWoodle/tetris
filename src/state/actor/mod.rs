use ggez::Context;

pub mod player;

use crate::settings::Settings;
use crate::tetrimino::TileType;
use crate::map::Map;

#[derive(Copy, Clone, PartialEq)]
pub enum Action {
    RotateLeft,
    RotateRight,
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    Drop,
}

pub trait Actor {
    fn is_auto_drop(&self) -> bool {
        true
    }

    fn on_spawn(&mut self, _settings: &Settings, _map: &Map, _current: TileType, _next: TileType, _score: usize, _lines: usize, _level: usize) {
    }

    fn check(&mut self, ctx: &mut Context, action: Action) -> bool;
    fn push(&mut self, _action: Action) {
    }

    fn update(&mut self, ctx: &mut Context);
}