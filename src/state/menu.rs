//use ggez::{
//    timer,
//    mint::Point2,
//    event::{KeyCode, KeyMods},
//    graphics::{
//        self, DrawParam, Text, Font, Scale, FilterMode,
//        spritebatch::SpriteBatch,
//    },
//    Context, GameResult,
//};
//use std::cmp;
//
//use crate::tetrimino::{TileType, Tetrimino};
//use crate::settings::{self, Settings, Bounds};
//use crate::random::{self, RandomGenerator};
//use super::{State, Resources, StateID};
//use super::actor::{
//    Action, Actor,
//    player::Player,
//};
//
//pub struct Menu {
//}
//
//impl State for Menu {
//    fn update(&mut self, ctx: &mut Context, settings: &Settings) -> GameResult<StateID> {
//        Ok(State::Menu)
//    }
//
//    fn draw(&mut self, ctx: &mut Context, settings: &Settings, res: &Resources) -> GameResult<()> {
//        Ok(())
//    }
//
//    fn key_down_event(&mut self, _ctx: &mut Context, settings: &Settings, keycode: KeyCode, _keymods: KeyMods, repeat: bool) -> StateID {
//        State::Menu
//    }
//}