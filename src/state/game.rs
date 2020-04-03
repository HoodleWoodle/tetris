use ggez::{
    timer,
    mint::Point2,
    event::{KeyCode, KeyMods},
    graphics::{
        self, DrawParam, Text, Font, Scale, FilterMode, Color,
        spritebatch::SpriteBatch,
    },
    Context, GameResult,
};
use std::cmp;

use crate::tetrimino::{TileType, Tetrimino};
use crate::settings::{self, Settings, Bounds};
use crate::random::{self, RandomGenerator};
use crate::map::Map;
use super::{State, Resources, StateID};
use super::actor::{
    Action, Actor,
    player::Player,
};

struct GameInstance {
    actor: Player,

    gen: Box<dyn RandomGenerator>,

    map: Map,
    current: Tetrimino,
    next: Tetrimino,

    score: usize,
    lines: usize,
    level: usize,

    line_counter: isize,

    player_text: Text,
    score_text: Text,
    lines_text: Text,
    level_text: Text,
    next_text: Text,

    drop_timer: Option<usize>,
    spawn_delay_timer: Option<usize>,
    animation_timer: Option<usize>,

    soft_drop: bool,
    animation_info: Vec<usize>,

    left_timer: Option<usize>,
    right_timer: Option<usize>,
}

impl GameInstance {
    fn new(settings: &Settings, res: &Resources, seed: [u8; 32], player: String, _left: bool) -> GameInstance {
        let mut actor = Player::new();

        let mut gen = random::create(seed, settings.random_generator);
        let current = Tetrimino::new(gen.next());
        let next = Tetrimino::new(gen.next());
        
        let start_level = settings.start_level;
        let level = start_level as isize;
        let line_counter = cmp::min(level * 10 + 10, cmp::max(100, level * 10 - 50)) as isize;

        let map = Map::new();
        actor.on_spawn(settings, &map, current.tile_type, next.tile_type, 0, 0, start_level);
 
        let mut player_text = Text::new(player);
        let mut score_text = Text::new("SCORE");
        let mut lines_text = Text::new("LINES");
        let mut level_text = Text::new("LEVEL");
        let mut next_text = Text::new("NEXT");

        player_text.set_font(res.font, Scale::uniform(settings.font.size_player));
        score_text.set_font(res.font, Scale::uniform(settings.font.size_default));
        lines_text.set_font(res.font, Scale::uniform(settings.font.size_default));
        level_text.set_font(res.font, Scale::uniform(settings.font.size_default));
        next_text.set_font(res.font, Scale::uniform(settings.font.size_default));

        GameInstance {
            actor,
            
            gen,

            map,
            current,
            next,

            score: 0,
            lines: 0,
            level: start_level,

            line_counter,

            player_text,
            score_text,
            lines_text,
            level_text,
            next_text,

            drop_timer: Some(GameInstance::gravity_value(start_level)),
            spawn_delay_timer: None,
            animation_timer: None,

            soft_drop: false,
            animation_info: Vec::new(),

            left_timer: None,
            right_timer: None,
        }
    }

    pub fn gravity_value(level: usize) -> usize {
        match level {
            0 => 48,
            1 => 43,
            2 => 38,
            3 => 33,
            4 => 28,
            5 => 23,
            6 => 18,
            7 => 13,
            8 => 8,
            9 => 6,
            10 => 5,
            11 => 5,
            12 => 5,
            13 => 4,
            14 => 4,
            15 => 4,
            16 => 3,
            17 => 3,
            18 => 3,
            19 => 2,
            20 => 2,
            21 => 2,
            22 => 2,
            23 => 2,
            24 => 2,
            25 => 2,
            26 => 2,
            27 => 2,
            28 => 2,
            _ => 1,
        }
    }

    fn rotate(&mut self, settings: &Settings, right: bool) {
        self.current.rotate(settings, &self.map, right);
    }

    fn rotate_left(&mut self, settings: &Settings) {
        self.actor.push(Action::RotateLeft);
        self.rotate(settings, false);
    }

    fn rotate_right(&mut self, settings: &Settings) {
        self.actor.push(Action::RotateRight);
        self.rotate(settings, true);
    }

    fn mov(&mut self, x_off: f32, y_off: f32) -> bool {
        self.current.mov(&self.map, x_off, y_off)
    }

    fn left(&mut self) {
        self.actor.push(Action::MoveLeft);
        self.mov(-1.0, 0.0);
    }

    fn right(&mut self) {
        self.actor.push(Action::MoveRight);
        self.mov(1.0, 0.0);
    }

    fn drop(&mut self) -> bool {
        self.actor.push(Action::Drop);
        self.mov(0.0, 1.0)
    }

    fn hard_drop(&mut self, settings: &Settings) {
        if settings.hard_drop_enabled {
            while self.drop() {}
        }
    }

    fn update_score(&mut self, complete_lines: usize) {
        self.lines += complete_lines;
        self.line_counter -= complete_lines as isize;

        if self.line_counter <= 0 {
            self.level += 1;
            self.line_counter = 10 + self.line_counter;
        }

        let factor = match complete_lines {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => panic!("dead code"),
        };
        self.score += factor * (self.level + 1);
    }

    fn update_drop(&mut self) {
        // tetrimino -> map
        self.map.apply(&self.current);

        // check for complete lines
        self.animation_info = self.map.complete_lines();
        if !self.animation_info.is_empty() {
            // update score
            self.update_score(self.animation_info.len() - 1);

            // trigger animation
            self.animation_timer = Some(20);
        } else {
            // trigger spawn delay
            self.spawn_delay_timer = Some(10); // TODO: correct value
        }

        self.drop_timer = None;

        // game over
        if self.map.collision(&self.next) {
            // TODO
            panic!("Score: {}", self.score);
        }
    }

    fn update(&mut self, ctx: &mut Context, settings: &Settings) {
        // gravity
        if self.actor.is_auto_drop() {
            if let Some(timer) = self.drop_timer {
                if timer == 0 {
                    if !self.drop() {
                        self.update_drop();
                    } else {
                        // reset drop timer
                        self.drop_timer = Some(GameInstance::gravity_value(self.level));
                    }
                } else {
                    if self.soft_drop && timer >= 2 {
                        self.drop_timer = Some(timer - 2);
                    }
                    else {
                        self.drop_timer = Some(timer - 1);
                    }
                }
            }
        } else {
            if self.actor.check(ctx, Action::Drop) {
                if !self.drop() {
                    self.update_drop();
                }
            }
        }

        // clear line animation
        if let Some(timer) = self.animation_timer {
            if timer == 0 {
                self.map.clear(&self.animation_info);

                // trigger spawn delay
                self.spawn_delay_timer = Some(10); // TODO: correct value

                self.animation_timer = None;
            } else {
                if timer % 4 == 0 {
                    // advance animation
                    let count = self.animation_info.len() - 1;

                    let step = timer / 4;
                    let x0 = step - 1;
                    let x1 = settings::MAP_WIDTH - step;
                    for i in 0..count {
                        let y = self.animation_info[i];
                        self.map.set(x0, y, TileType::Empty);
                        self.map.set(x1, y, TileType::Empty);
                    }
                }

                self.animation_timer = Some(timer - 1);
            } 
        }

        // ARE (spawn delay)
        if let Some(timer) = self.spawn_delay_timer {
            if timer == 0 {
                // spawn tetrimino
                self.current = self.next.clone();
                self.next = Tetrimino::new(self.gen.next());

                self.actor.on_spawn(settings, &self.map, self.current.tile_type, self.next.tile_type, self.score, self.lines, self.level);
                
                // reset drop timer
                self.drop_timer = Some(GameInstance::gravity_value(self.level));

                self.spawn_delay_timer = None;
            } else {
                self.spawn_delay_timer = Some(timer - 1);
            }
        }

        self.actor.update(ctx);
    }

    fn input(&mut self, ctx: &mut Context, settings: &Settings) {
        self.soft_drop = self.actor.check(ctx, Action::SoftDrop);

        if self.actor.check(ctx, Action::MoveLeft) {
            if let Some(timer) = self.left_timer {
                if timer == 0 {
                    self.left();
                    self.left_timer = Some(6);
                } else {
                    self.left_timer = Some(timer - 1);
                }
            } else {
	            self.left();
                self.left_timer = Some(16);
            }
	    } else {
	        self.left_timer = None;
        }

        if self.actor.check(ctx, Action::MoveRight) {
            if let Some(timer) = self.right_timer {
                if timer == 0 {
                    self.right();
                    self.right_timer = Some(6);
                } else {
                    self.right_timer = Some(timer - 1);
                }
            } else {
	            self.right();
                self.right_timer = Some(16);
            }
	    } else {
	        self.right_timer = None;
        }

        //KeyCode::Shift => self.instance.hold(),
        //KeyCode::C => self.instance.hold(),

        if self.actor.check(ctx, Action::RotateLeft) {
            self.rotate_left(settings);
        }

        if self.actor.check(ctx, Action::RotateRight) {
            self.rotate_right(settings);
        }

        if self.actor.check(ctx, Action::HardDrop) {
            self.hard_drop(settings)
        }
    }

    fn draw(&self, ctx: &mut Context, settings: &Settings, batch: &mut SpriteBatch, font: Font, color: Color) -> GameResult<()> {
        let map_position = &settings.map_positions[0];
        let next_bounds = &settings.next_bounds[0];
        let player_bounds = &settings.player_bounds[0];
        let score_bounds = &settings.score_bounds[0];
        let lines_bounds = &settings.lines_bounds[0];
        let level_bounds = &settings.level_bounds[0];

        self.map.draw(settings, batch, color, self.level, map_position);
        
        if self.drop_timer != None {
            self.current.draw_map(settings, batch, color, self.level, map_position);
        }

        GameInstance::draw_text(ctx, color, player_bounds, &self.player_text);
        
        let h = 2.0 * settings.font.next_text_y_offset + self.next_text.height(ctx) as f32;
        let bounds = Bounds {
            x: next_bounds.x,
            y: next_bounds.y,
            w: next_bounds.w,
            h,
        };
        GameInstance::draw_text(ctx, color, &bounds, &self.next_text);
        let x = next_bounds.x + next_bounds.w / 2.0;
        let y = next_bounds.y + bounds.h + (next_bounds.h - bounds.h) / 2.0;
        self.next.draw(settings, batch, color, self.level, Point2 { x, y });
        
        GameInstance::draw_text_and_value(ctx, settings, font, color, score_bounds, &self.score_text, self.score);
        GameInstance::draw_text_and_value(ctx, settings, font, color, lines_bounds, &self.lines_text, self.lines);
        GameInstance::draw_text_and_value(ctx, settings, font, color, level_bounds, &self.level_text, self.level);

        Ok(())
    }

    fn draw_text(ctx: &mut Context, color: Color, bounds: &Bounds, text: &Text) {
        let x = bounds.x + (bounds.w - text.width(ctx) as f32) / 2.0;
        let y = bounds.y + (bounds.h - text.height(ctx) as f32) / 2.0;
               
        graphics::queue_text(ctx, &text, Point2 { x, y }, Some(color));
    }

    fn draw_text_and_value(ctx: &mut Context, settings: &Settings, font: Font, color: Color, bounds: &Bounds, text: &Text, val: usize) {
        let y = bounds.y + bounds.h / 3.0;
        let new_bounds = Bounds {
            x: bounds.x,
            y,
            w: bounds.w,
            h: 0.0
        };
        GameInstance::draw_text(ctx, color, &new_bounds, text);

        let mut text = Text::new(val.to_string());
        text.set_font(font, Scale::uniform(settings.font.size_default));
        let y = bounds.y + bounds.h * 2.0 / 3.0;
        let new_bounds = Bounds {
            x: bounds.x,
            y,
            w: bounds.w,
            h: 0.0
        };
        GameInstance::draw_text(ctx, color, &new_bounds, &text);
    }
}

pub struct GameState {
    batch: SpriteBatch,

    instance: GameInstance,

    running: bool,
}

impl GameState {
    pub fn new(settings: &Settings, res: &Resources, seed: [u8; 32]) -> GameResult<GameState> {
        let batch = SpriteBatch::new(res.tileset.clone());

        let instance = GameInstance::new(settings, res, seed, settings.nickname.clone(), true);

        let state = GameState {
            batch,

            instance,

            running: true,
        };

        Ok(state)
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context, settings: &Settings) -> GameResult<StateID> {
        while timer::check_update_time(ctx, 60) {
            if self.running {
                self.instance.input(ctx, settings);
                self.instance.update(ctx, settings);
            }
        }
        
        Ok(StateID::Game)
    }

    fn draw(&mut self, ctx: &mut Context, settings: &Settings, res: &Resources) -> GameResult<()> {
        let color = if self.running {
            graphics::WHITE
        } else {
            Color::new(0.5, 0.5, 0.5, 1.0)
        };
        let draw_param = DrawParam::default()
            .color(color);

        self.instance.draw(ctx, settings, &mut self.batch, res.font, color)?;
        
        // actual draw calls
        graphics::draw(ctx, &res.background, draw_param)?;

        graphics::draw(ctx, &self.batch, draw_param)?;
        self.batch.clear();

        graphics::draw_queued_text(ctx, draw_param, None, FilterMode::Nearest)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, settings: &Settings, keycode: KeyCode, _keymods: KeyMods, repeat: bool) -> StateID {
        if !repeat {
            if !settings.multiplayer_enabled {
                match keycode {
                    KeyCode::Escape => self.running = !self.running,
                    KeyCode::F1 => self.running = !self.running,
                    KeyCode::P => self.running = !self.running,
        
                    _ => (),
                }
            }
        }

        StateID::Game
    }
}
