use ggez::{
    timer,
    mint::Point2,
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color, DrawParam, Rect, Image},
    audio::{Source, SoundSource},
	Context, ContextBuilder, GameResult,
};
use rand::{
    rngs::StdRng,
    RngCore, SeedableRng,
};
use std::{
    process,
    cmp::PartialEq,
    path::Path,
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher,
    time::{Duration, SystemTime},
    env,
    path,
};

const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 22;
const MAP_TILE_COUNT: usize = MAP_WIDTH * MAP_HEIGHT;

const MAP_OFFSET: f32 = 2.0;
const TILE_SIZE: f32 = 32.0;

const WINDOW_WIDTH: f32 = TILE_SIZE * (MAP_WIDTH as f32) + MAP_OFFSET * 2.0;
const WINDOW_HEIGHT: f32 = TILE_SIZE * ((MAP_HEIGHT as f32) - 2.0) + MAP_OFFSET * 2.0;

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
    // - next tile
    // - line, score, level indicator

    // TO TIME:
    // - variable speed
    // - line removal schoener darstellen
    // - soft drop
    // - hard drop
    // - game over + retry screen

    // OPTIONAL:
    // - max out above 999999
    // - shadowing
    // - hold piece
    // - texture pack change (lvl) (s. SETTINGS)

    // BUGS & FIXES & ...:
    // - quirin´s line problem
    // - save generators history locally (only one generator)
    // - window scaling based on monitor (correct texture scaling)

    // SETTINGS:
    // - OPTIONALS
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
        Err(e) => eprintln!("Error occuZ: {}", e),
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
    fn draw(self, ctx: &mut Context, res: &Resources, pos: Point2<f32>) -> GameResult<()> {
        if pos.y < 2.0 {
            return Ok(());
        }

        let x = MAP_OFFSET + pos.x * TILE_SIZE;
        let y = MAP_OFFSET + (pos.y - 2.0) * TILE_SIZE;

        let rect = Rect::new(((self as i32) as f32) * 0.125, 0.0, 0.125, 1.0);
        let draw_param = DrawParam::default()
            .src(rect)
            .dest(Point2 { x, y });
        
        graphics::draw(ctx, &res.image, draw_param)
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

    fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        for &pos in self.tiles.iter() {
            let final_pos = Point2 { x: (self.pos.x + pos.x), y: (self.pos.y + pos.y) };
            self.tile_type.draw(ctx, res, final_pos)?;
        }

        Ok(())
    }
}

struct Resources {
    _sound: Source,
    image: Image,
}

impl Resources {
    fn new(ctx: &mut Context) -> GameResult<Resources> {
        let mut _sound = Source::new(ctx, Path::new("/sound.ogg"))?;
        _sound.set_repeat(true);
        _sound.set_volume(0.025);
        _sound.play()?;

        let image = Image::new(ctx, Path::new("/tiles.png"))?;

        let res = Resources {
            _sound,
            image,
        };

        Ok(res)
    }
}

struct GameState {
    res: Resources,
    
    timer: Duration,
    instance: GameInstance
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let seed = GameState::generate_seed();

        let state = GameState {
            res: Resources::new(ctx)?,
            
            timer: Duration::default(),
            instance: GameInstance::new("Player 1".to_owned(), seed),
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

struct GameInstance {
    _player: String,

    gen: RandomGenerator,

    map: [TileType; MAP_TILE_COUNT],
    current: Tetrimino,
    next: TileType,

    score: usize,
    lines: usize,
    level: usize,
}

impl GameInstance {
    pub fn new(player: String, seed: [u8; 32]) -> GameInstance {
        let mut gen = RandomGenerator::new(seed);
        let current = Tetrimino::new(gen.next());
        let next = gen.next();

        GameInstance {
            _player: player,

            gen,

            map: [TileType::Empty; MAP_TILE_COUNT],
            current,
            next,

            score: 0,
            lines: 0,
            level: 0,
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
        if self.mov(0.0, 1.0) {
            return true;
        }

        for &pos in self.current.tiles.iter() {
            let x = (self.current.pos.x + pos.x).round() as usize;
            let y = (self.current.pos.y + pos.y).round() as usize;

            self.map[MAP_WIDTH * y + x] = self.current.tile_type;
        }

        let complete_lines = self.cleanup_map();
        self.update_score(complete_lines);

        let new_tet = Tetrimino::new(self.next);
        self.next = self.gen.next();
        
        if new_tet.collision(&self.map) {
            // TODO
            panic!("Score: {}", self.score);
        }

        self.current = new_tet;
        false
    }

    fn drop_hard(&mut self) {
        while self.drop() {}
    }

    fn cleanup_map(&mut self) -> usize {
        let mut count = 0;
        let mut lines = [0; 5];

        // count complete lines
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

        // remove complete lines
        for i in 0..count {
            for y in (lines[i + 1]..lines[i]).rev() {
                for x in 0..MAP_WIDTH {
                    self.map[MAP_WIDTH * (y + i + 1) + x] = self.map[MAP_WIDTH * y + x];
                }
            }
        }

        for y in 0..count {
            for x in 0..MAP_WIDTH {
                self.map[MAP_WIDTH * y + x] = TileType::Empty;
            }
        }

        count
    }

    fn update_score(&mut self, complete_lines: usize) {
        if complete_lines == 0 {
            return;
        }

        self.lines += complete_lines;
        self.level = self.lines / 10;

        let factor = match complete_lines {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => panic!("dead code"),
        };
        self.score += factor * (self.level + 1);
    }

    fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let pos: Point2<f32>  = Point2 { x: x as f32, y: y as f32 };
                self.map[y * MAP_WIDTH + x].draw(ctx, res, pos)?;
            }
        }

        self.current.draw(ctx, res)?;
        
        let title = String::from("Tetris - Score: ") + &self.score.to_string();
        graphics::window(ctx).set_title(&title);

        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.timer += timer::delta(ctx);

        while self.timer.as_millis() >= 250 {
            self.instance.drop();
            self.timer -= Duration::from_millis(250);
        }

        timer::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.75, 0.75, 0.75, 1.0));

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
