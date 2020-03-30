use ggez::{
    timer,
    mint::Point2,
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, DrawParam, Rect, Image, Text, Font, Scale},
    audio::{Source, SoundSource},
    Context, ContextBuilder, GameResult,
};
use rand::{
    rngs::StdRng,
    RngCore, SeedableRng,
};
use std::{
    process,
    cmp::{self, PartialEq},
    path::Path,
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher,
    time::SystemTime,
    env,
    path,
};

const START_LEVEL: usize = 8;

const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 22;
const MAP_TILE_COUNT: usize = MAP_WIDTH * MAP_HEIGHT;

const TILE_SIZE: f32 = 32.0;

const WINDOW_WIDTH: f32 = 572.0;
const WINDOW_HEIGHT: f32 = 753.0;

const DEFAULT_FONT_SIZE: f32 = 40.0;
const PLAYER_FONT_SIZE: f32 = 48.0;

const NEXT_TEXT_Y_OFFSET: f32 = 24.0;

const PLAYER_BOUNDS: [Rect; 1] = [Rect::new(16.0, 16.0, 544.0, 60.0)];
const SCORE_BOUNDS: [Rect; 1] = [Rect::new(16.0, 91.0, 200.0, 96.0)];
const LINES_BOUNDS: [Rect; 1] = [Rect::new(16.0, 196.0, 200.0, 96.0)];
const LEVEL_BOUNDS: [Rect; 1] = [Rect::new(16.0, 301.0, 200.0, 96.0)];
const NEXT_BOUNDS: [Rect; 1] = [Rect::new(66.0, 412.0, 150.0, 200.0)];
const MAP_POSITION: [Point2<f32>; 1] = [Point2 { x: 234.0, y: 94.0 }];

const WALL_KICK_DATA_TSZJL: [[[(f32, f32); 5]; 4]; 2] = [
    [
        // Deg0 >> Deg90
        [( 0.0, 0.0), (-1.0, 0.0), (-1.0, 1.0), ( 0.0,-2.0), (-1.0,-2.0)],
        // Deg90 >> Deg180
        [( 0.0, 0.0), ( 1.0, 0.0), ( 1.0,-1.0), ( 0.0, 2.0), ( 1.0, 2.0)],
        // Deg180 >> Deg270
        [( 0.0, 0.0), ( 1.0, 0.0), ( 1.0, 1.0), ( 0.0,-2.0), ( 1.0,-2.0)],
        // Deg270 >> Deg0
        [( 0.0, 0.0), (-1.0, 0.0), (-1.0,-1.0), ( 0.0, 2.0), (-1.0, 2.0)],
    ],
    [
        // Deg90 >> Deg0
        [(0.0, 0.0), ( 1.0, 0.0), ( 1.0,-1.0), ( 0.0, 2.0), ( 1.0, 2.0)],
        // Deg180 >> Deg90
        [(0.0, 0.0), (-1.0, 0.0), (-1.0, 1.0), ( 0.0,-2.0), (-1.0,-2.0)],
        // Deg270 >> Deg180
        [(0.0, 0.0), (-1.0, 0.0), (-1.0,-1.0), ( 0.0, 2.0), (-1.0, 2.0)],
        // Deg0 >> Deg270
        [(0.0, 0.0), ( 1.0, 0.0), ( 1.0, 1.0), ( 0.0,-2.0), ( 1.0,-2.0)],
    ],
];

const WALL_KICK_DATA_I: [[[(f32, f32); 5]; 4]; 2] = [
    [
        // Deg0 >> Deg90
        [( 0.0, 0.0), (-2.0, 0.0), ( 1.0, 0.0), (-2.0,-1.0), ( 1.0, 2.0)],
        // Deg90 >> Deg180
        [( 0.0, 0.0), (-1.0, 0.0), ( 2.0, 0.0), (-1.0, 2.0), ( 2.0,-1.0)],
        // Deg180 >> Deg270
        [( 0.0, 0.0), ( 2.0, 0.0), (-1.0, 0.0), ( 2.0, 1.0), (-1.0,-2.0)],
        // Deg270 >> Deg0
        [( 0.0, 0.0), ( 1.0, 0.0), (-2.0, 0.0), ( 1.0,-2.0), (-2.0, 1.0)],
    ],
    [
        // Deg90 >> Deg0
        [( 0.0, 0.0), ( 2.0, 0.0), (-1.0, 0.0), ( 2.0, 1.0), (-1.0,-2.0)],
        // Deg180 >> Deg90
        [( 0.0, 0.0), ( 1.0, 0.0), (-2.0, 0.0), ( 1.0,-2.0), (-2.0, 1.0)],
        // Deg270 >> Deg180
        [( 0.0, 0.0), (-2.0, 0.0), ( 1.0, 0.0), (-2.0,-1.0), ( 1.0, 2.0)],
        // Deg0 >> Deg270
        [( 0.0, 0.0), (-1.0, 0.0), ( 2.0, 0.0), (-1.0, 2.0), ( 2.0,-1.0)],
    ],
];

fn main() {
    // TODO:

    // ADITIONAL STATES:
    // - game over + retry + exit screen
    // - welcome screen
    // - help screen

    // TO TIME:
    // - soft drop
    // - DAS (initial delay 16 frames - then 6 frames)

    // OPTIONAL:
    // - hard drop
    // - shadowing
    // - hold piece
    // - texture pack change (lvl) (s. SETTINGS)

    // BUGS & FIXES & ...:
    // - disable audio player before exit
    // - quirin´s line problem
    // - save generators history locally (only one generator)
    // - window scaling bug on laptops?
    // - font unscharf

    // SETTINGS:
    // - OPTIONALS
    // - window size/scaling
    // - Load GUI offset file
    // - Sound on/off, volume
    // - texture pack (s. OPTIONAL)
    // - type of RandomGenerator

    let mut ctx_builder = ContextBuilder::new("tetris", "");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx_builder = ctx_builder.add_resource_path(path);
    }

    let window_mode = WindowMode::default()
        .dimensions(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let window_setup = WindowSetup::default()
        .title("Tetris")
        .icon("/icon.png");

    let (mut ctx, mut event_loop) = ctx_builder
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("Could not create ggez context!");

    let mut state = match GameState::new(&mut ctx) {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Could not create GameState: {}", e);
            process::exit(1);
        }
    };

    match event::run(&mut ctx, &mut event_loop, &mut state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
    Empty,
}

impl TileType {
    fn draw_map(self, ctx: &mut Context, res: &Resources, level: usize, map_position: &Point2<f32>, pos: Point2<f32>) -> GameResult<()> {
        if pos.y < 2.0 {
            return Ok(());
        }

        let x = map_position.x + pos.x * TILE_SIZE;
        let y = map_position.y + (pos.y - 2.0) * TILE_SIZE;

        self.draw(ctx, res, level, Point2 { x, y })
   }

    fn draw(self, ctx: &mut Context, res: &Resources, level: usize, pos: Point2<f32>) -> GameResult<()> {
        let rect = Rect::new(((self as i32) as f32) * 0.125, ((level % 10) as f32) * 0.1, 0.125, 0.1);
        let draw_param = DrawParam::default()
            .src(rect)
            .dest(pos);

       graphics::draw(ctx, &res.tileset, draw_param)
    }
}

struct RandomGenerator {
    rng: StdRng,
    types: Vec<TileType>,
}

impl RandomGenerator {
    fn new(seed: [u8; 32]) -> RandomGenerator {
        let generator = RandomGenerator {
            rng: StdRng::from_seed(seed),
            types: Vec::with_capacity(7),
        };

        generator
    }

    fn next(&mut self) -> TileType {
        let mut len = self.types.len();

        if len == 0 {
            self.types.extend_from_slice(&[
                TileType::I,
                TileType::O,
                TileType::T,
                TileType::S,
                TileType::Z,
                TileType::J,
                TileType::L,
            ]);
            len = 7;
        }

        if len == 1 {
            return self.types.pop().unwrap();
        }

        let value = (self.rng.next_u32() as usize) % len;
        self.types.swap_remove(value)
    }
}

#[derive(Copy, Clone)]
enum Orientation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Orientation {
    fn rotate(&self, clockwise: bool) -> Orientation {
        match self {
            Orientation::Deg0 => if clockwise { Orientation::Deg90 } else { Orientation::Deg270 },
            Orientation::Deg90 => if clockwise { Orientation::Deg180 } else { Orientation::Deg0 },
            Orientation::Deg180 => if clockwise { Orientation::Deg270 } else { Orientation::Deg90 },
            Orientation::Deg270 => if clockwise { Orientation::Deg0 } else { Orientation::Deg180 },
        }
    }
}

#[derive(Clone)]
struct Tetrimino {
    tile_type: TileType,
    pos: Point2<f32>,
    orientation: Orientation,
    tiles: [Point2<f32>; 4],
}

impl Tetrimino {
    fn new(tile_type: TileType) -> Tetrimino {
        match tile_type {
            TileType::I => Tetrimino::new_i(),
            TileType::O => Tetrimino::new_o(),
            TileType::T => Tetrimino::new_t(),
            TileType::S => Tetrimino::new_s(),
            TileType::Z => Tetrimino::new_z(),
            TileType::J => Tetrimino::new_j(),
            TileType::L => Tetrimino::new_l(),
            _ => panic!("dead code"),
        }
    }

    fn new_i() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::I,
            pos: Point2 { x: 4.5, y: 1.5 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x:  1.5, y: -0.5 },
                Point2 { x:  0.5, y: -0.5 },
                Point2 { x: -0.5, y: -0.5 },
                Point2 { x: -1.5, y: -0.5 },
            ],
        }
    }

    fn new_o() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::O,
            pos: Point2 { x: 4.5, y: 0.5 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x: -0.5, y: -0.5 },
                Point2 { x: -0.5, y:  0.5 },
                Point2 { x:  0.5, y: -0.5 },
                Point2 { x:  0.5, y:  0.5 },
            ],
        }
    }

    fn new_t() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::T,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x: -1.0, y:  0.0 },
                Point2 { x:  1.0, y:  0.0 },
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x:  0.0, y: -1.0 },
            ],
        }
    }

    fn new_s() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::S,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x: -1.0, y:  0.0 },
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x:  0.0, y: -1.0 },
                Point2 { x:  1.0, y: -1.0 },
            ],
        }
    }

    fn new_z() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::Z,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x: -1.0, y: -1.0 },
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x:  0.0, y: -1.0 },
                Point2 { x:  1.0, y:  0.0 },
            ],
        }
    }

    fn new_j() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::J,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x: -1.0, y:  0.0 },
                Point2 { x:  1.0, y:  0.0 },
                Point2 { x: -1.0, y: -1.0 },
            ],
        }
    }

    fn new_l() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::L,
            pos: Point2 { x: 4.0, y: 1.0 },
            orientation: Orientation::Deg0,
            tiles: [
                Point2 { x:  0.0, y:  0.0 },
                Point2 { x: -1.0, y:  0.0 },
                Point2 { x:  1.0, y:  0.0 },
                Point2 { x:  1.0, y: -1.0 },
            ],
        }
    }

    fn mov(&mut self, map: &[TileType; MAP_TILE_COUNT], x_off: f32, y_off: f32) -> bool {
        self.pos.x += x_off;
        self.pos.y += y_off;

        if self.collision(map)
        {
            self.pos.x -= x_off;
            self.pos.y -= y_off;
            return false;
        }

        true
    }

    fn rotate(&mut self, map: &[TileType; MAP_TILE_COUNT], clockwise: bool) -> bool {
        if self.tile_type == TileType::O {
            return true;
        }

        let mut new_tet = Tetrimino {
            tile_type: self.tile_type,
            pos: self.pos,
            orientation: self.orientation.rotate(clockwise),
            tiles: [Point2 { x: 0.0, y: 0.0 }; 4],
        };

        for i in 0..4 {
            // clockwise rotation
            // (1,0) -> (0,1)
            // (0,1) -> (-1,0)
            // A x = x'
            // A = [ [0,-1], [1,0]]

            let mut x = self.tiles[i].y;
            let mut y = -self.tiles[i].x;

            if clockwise {
                x = -x;
                y = -y;
            }

            new_tet.tiles[i] = Point2 { x, y };
        }

        if new_tet.collision(map) {
            // wall kicks
            let rotation_direction_index = if clockwise { 0 } else { 1 };
            let orientation_index = self.orientation as usize;

            let data = if self.tile_type == TileType::I {
                &WALL_KICK_DATA_I[rotation_direction_index][orientation_index]
            } else {
                &WALL_KICK_DATA_TSZJL[rotation_direction_index][orientation_index]
            };

            for i in 0..5 {
                if new_tet.mov(map, data[i].0, data[i].1) {
                    *self = new_tet;
                    return true;
                }
            }

            return false;
        }

        *self = new_tet;
        true
    }

    fn collision(&self, map: &[TileType; MAP_TILE_COUNT]) -> bool {
        for &tile in self.tiles.iter() {
            let x = (self.pos.x + tile.x).round() as usize;
            let y = (self.pos.y + tile.y).round() as usize;

            // (x < 0 || y < 0) is tested within next check because of usize wrap-around

            if x >= MAP_WIDTH || y >= MAP_HEIGHT {
                return true;
            }

            if map[y * MAP_WIDTH + x] != TileType::Empty {
                return true;
            }
        }

        false
    }

    fn draw_map(&self, ctx: &mut Context, res: &Resources, level: usize, map_position: &Point2<f32>) -> GameResult<()> {
        for &pos in self.tiles.iter() {
            let final_pos = Point2 { x: (self.pos.x + pos.x), y: (self.pos.y + pos.y) };
            self.tile_type.draw_map(ctx, res, level, map_position, final_pos)?;
        }

        Ok(())
    }

    fn draw(&self, ctx: &mut Context, res: &Resources, level: usize, offset: Point2<f32>) -> GameResult<()> {
        for &pos in self.tiles.iter() {
            let final_pos = Point2 { x: (offset.x + (pos.x - 0.5) * TILE_SIZE), y: (offset.y + (pos.y - 0.5) * TILE_SIZE) };
            self.tile_type.draw(ctx, res, level, final_pos)?;
        }
        Ok(())
    }
}

struct Resources {
    _sound: Source,
    tileset: Image,
    background: Image,
    font: Font,
}

impl Resources {
    fn new(ctx: &mut Context) -> GameResult<Resources> {
        let mut _sound = Source::new(ctx, Path::new("/sound.ogg"))?;
        _sound.set_repeat(true);
        _sound.set_volume(0.025);
        _sound.play()?;

        let tileset = Image::new(ctx, Path::new("/tileset_nes.png"))?;
        let background = Image::new(ctx, Path::new("/background.png"))?;
        let font = Font::new(ctx, Path::new("/font.ttf"))?;

        let res = Resources {
            _sound,
            tileset,
            background,
            font,
        };

        Ok(res)
    }
}

struct GameInstance {
    gen: RandomGenerator,

    map: [TileType; MAP_TILE_COUNT],
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

    animation_info: Option<(usize, [usize; 5])>
}

impl GameInstance {
    fn new(res: &Resources, seed: [u8; 32], start_level: usize, player: String, _left: bool) -> GameInstance {
        let mut gen = RandomGenerator::new(seed);
        let current = Tetrimino::new(gen.next());
        let next = Tetrimino::new(gen.next());
        
        let level = start_level as isize;
        let line_counter = cmp::min(level * 10 + 10, cmp::max(100, level * 10 - 50)) as isize;
 
        let mut player_text = Text::new(player);
        let mut score_text = Text::new("SCORE");
        let mut lines_text = Text::new("LINES");
        let mut level_text = Text::new("LEVEL");
        let mut next_text = Text::new("NEXT");

        player_text.set_font(res.font, Scale::uniform(PLAYER_FONT_SIZE));
        score_text.set_font(res.font, Scale::uniform(DEFAULT_FONT_SIZE));
        lines_text.set_font(res.font, Scale::uniform(DEFAULT_FONT_SIZE));
        level_text.set_font(res.font, Scale::uniform(DEFAULT_FONT_SIZE));
        next_text.set_font(res.font, Scale::uniform(DEFAULT_FONT_SIZE));

        GameInstance {
            gen,

            map: [TileType::Empty; MAP_TILE_COUNT],
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

            animation_info: None,
        }
    }

    fn gravity_value(level: usize) -> usize {
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

    fn rotate(&mut self, clockwise: bool) {
        self.current.rotate(&self.map, clockwise);
    }

    fn mov(&mut self, x_off: f32, y_off: f32) -> bool {
        self.current.mov(&self.map, x_off, y_off)
    }

    fn left(&mut self) {
        self.mov(-1.0, 0.0);
    }

    fn right(&mut self) {
        self.mov(1.0, 0.0);
    }

    fn drop(&mut self) -> bool {
        self.mov(0.0, 1.0)
    }

    fn drop_hard(&mut self) {
        while self.drop() {}
    }

    fn complete_lines(&mut self) -> Option<(usize, [usize; 5])> {
        let mut count = 0;
        let mut lines = [0; 5];

        for y in (0..MAP_HEIGHT).rev() {
            let mut complete = true;

            for x in 0..MAP_WIDTH {
                if self.map[MAP_WIDTH * y + x] == TileType::Empty {
                    complete = false;
                    break;
                }
            }

            if complete {
                lines[count] = y;
                count += 1;
            }
        }

        if count != 0 {
            Some((count, lines))
        } else {
            None
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

    fn draw_text(ctx: &mut Context, bounds: &Rect, text: &Text) -> GameResult<()> {
        let x = bounds.x + (bounds.w - text.width(ctx) as f32) / 2.0;
        let y = bounds.y + (bounds.h - text.height(ctx) as f32) / 2.0;
        let draw_param = DrawParam::default()
            .dest(Point2 { x, y });
       	    
        graphics::draw(ctx, text, draw_param)
    }

    fn draw_text_and_value(ctx: &mut Context, res: &Resources,  bounds: &Rect, text: &Text, val: usize) -> GameResult<()> {
        let y = bounds.y + bounds.h / 3.0;
        let new_bounds = Rect::new(bounds.x, y, bounds.w, 0.0);
        GameInstance::draw_text(ctx, &new_bounds, text)?;

        let mut text = Text::new(val.to_string());
        text.set_font(res.font, Scale::uniform(DEFAULT_FONT_SIZE));
        let y = bounds.y + bounds.h * 2.0 / 3.0;
        let new_bounds = Rect::new(bounds.x, y, bounds.w, 0.0);
        GameInstance::draw_text(ctx, &new_bounds, &text)
    }

    fn update(&mut self) {
        // gravity
        if let Some(timer) = self.drop_timer {
            if timer == 0 {
                if !self.drop() {
                    // tetrimino -> map
                    for &pos in self.current.tiles.iter() {
                        let x = (self.current.pos.x + pos.x).round() as usize;
                        let y = (self.current.pos.y + pos.y).round() as usize;

                        self.map[MAP_WIDTH * y + x] = self.current.tile_type;
                    }

                    // check for complete lines
                    self.animation_info = self.complete_lines();
                    if let Some((count, _)) = self.animation_info {
                        // update score
                        self.update_score(count);
    
                        // trigger animation
                        self.animation_timer = Some(20);
                    } else {
                        // trigger spawn delay
                        self.spawn_delay_timer = Some(10); // TODO: correct value
                    }

                    self.drop_timer = None;
    
                    // game over
                    if self.next.collision(&self.map) {
                        // TODO
                        panic!("Score: {}", self.score);
                    }
                } else {
                    // reset drop timer
                    self.drop_timer = Some(GameInstance::gravity_value(self.level));
                }
            } else {
                self.drop_timer = Some(timer - 1);
            }
        }

        // clear line animation
        if let Some(timer) = self.animation_timer {
            let (count, lines) = self.animation_info.unwrap();

            if timer == 20 {
                // remove complete lines
                for i in 0..count {
                    println!("{} {} {}", count, lines[i+1], lines[i]);
                    for y in (lines[i + 1]..lines[i]).rev() {
                        for x in 0..MAP_WIDTH {
                            self.map[MAP_WIDTH * (y + i + 1) + x] = self.map[MAP_WIDTH * y + x];
                        }
                    }
                }
        
                //
                for i in 0..count {
                    for x in 0..MAP_WIDTH {
                        self.map[MAP_WIDTH * i + x] = TileType::Empty;
                    }
                }

                // trigger spawn delay
                self.spawn_delay_timer = Some(10); // TODO: correct value

                self.animation_timer = None;
            } else {
                if timer % 4 == 0 {
                    // advance animation
                    let step = timer / 4;
                    let x0 = step - 1;
                    let x1 = MAP_WIDTH - step;
                    for i in 0..count {
                        self.map[MAP_WIDTH * lines[i] + x0] = TileType::Empty;
                        self.map[MAP_WIDTH * lines[i] + x1] = TileType::Empty;
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

                // reset drop timer
                self.drop_timer = Some(GameInstance::gravity_value(self.level));

                self.spawn_delay_timer = None;
            } else {
                self.spawn_delay_timer = Some(timer - 1);
            }
        }
    }

    fn input(&mut self, _ctx: &mut Context) {
        // inputs
        // DAS
    }

    fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        let map_position = &MAP_POSITION[0];
        let next_bounds = &NEXT_BOUNDS[0];
        let player_bounds = &PLAYER_BOUNDS[0];
        let score_bounds = &SCORE_BOUNDS[0];
        let lines_bounds = &LINES_BOUNDS[0];
        let level_bounds = &LEVEL_BOUNDS[0];

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let pos: Point2<f32>  = Point2 { x: x as f32, y: y as f32 };
                self.map[y * MAP_WIDTH + x].draw_map(ctx, res, self.level, map_position, pos)?;
            }
        }

        self.current.draw_map(ctx, res, self.level, map_position)?;

        GameInstance::draw_text(ctx, player_bounds, &self.player_text)?;

        let h = 2.0 * NEXT_TEXT_Y_OFFSET + self.next_text.height(ctx) as f32;
        let bounds = Rect::new(next_bounds.x, next_bounds.y, next_bounds.w, h);
        GameInstance::draw_text(ctx, &bounds, &self.next_text)?;
        let x = next_bounds.x + next_bounds.w / 2.0;
        let y = next_bounds.y + bounds.h + (next_bounds.h - bounds.h) / 2.0;
        self.next.draw(ctx, res, self.level, Point2 { x, y })?;

        GameInstance::draw_text_and_value(ctx, res, score_bounds, &self.score_text, self.score)?;
        GameInstance::draw_text_and_value(ctx, res, lines_bounds, &self.lines_text, self.lines)?;
        GameInstance::draw_text_and_value(ctx, res, level_bounds, &self.level_text, self.level)?;

        Ok(())
    }
}

struct GameState {
    res: Resources,

    instance: GameInstance
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
	let res =  Resources::new(ctx)?;
        let seed = GameState::generate_seed();
        let instance = GameInstance::new(&res, seed, START_LEVEL, "PLAYER 1".to_owned(), true);

        let state = GameState {
            res,

            instance,
        };

        Ok(state)
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

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        println!("{:?}", timer::fps(ctx));

        while timer::check_update_time(ctx, 60) {
            self.instance.input(ctx);
            self.instance.update();
        }
        
        timer::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.res.background, DrawParam::default())?;

        self.instance.draw(ctx, &self.res)?;

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool, ) {
        if !repeat {
            match keycode {
                KeyCode::Up => self.instance.rotate(true),
                KeyCode::X => self.instance.rotate(true),

                KeyCode::Space => self.instance.drop_hard(),
                //KeyCode::Down => self.instance.soft_drop(),

                //KeyCode::Shift => self.instance.hold(),
                //KeyCode::C => self.instance.hold(),

                KeyCode::RControl => self.instance.rotate(false),
                KeyCode::Y => self.instance.rotate(false),

                //KeyCode::Escape => pause(),
                //KeyCode::F1 => pause(),

                KeyCode::Left => self.instance.left(),
                KeyCode::Right => self.instance.right(),
                _ => (),
            }
        }
    }
}
