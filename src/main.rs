use ggez::{
    timer,
    mint::Point2,
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color, DrawParam, Rect, Image},
    audio::{Source, SoundSource},
	Context, ContextBuilder, GameResult,
};
use rand::{self, Rng};
use std::{
    process,
    cmp::PartialEq,
    time::Duration,
    path::Path,
    env,
    path,
};

const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 22;
const FIELD_TILE_COUNT: usize = FIELD_WIDTH * FIELD_HEIGHT;

const FIELD_OFFSET: f32 = 2.0;
const TILE_SIZE: f32 = 32.0;

const WINDOW_WIDTH: f32 = TILE_SIZE * (FIELD_WIDTH as f32) + FIELD_OFFSET * 2.0;
const WINDOW_HEIGHT: f32 = TILE_SIZE * ((FIELD_HEIGHT as f32) - 2.0) + FIELD_OFFSET * 2.0;

fn main() {
    // TODO:
    // - Game over + Retry Screen

    // - Refactoring
    // - line removal schoener darstellen
    // - Hard drop (nicht sooo hard)
    // - Soft drop

    let mut ctx_builder = ContextBuilder::new("tetris", "");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx_builder = ctx_builder.add_resource_path(path);
    }

    let window_mode = WindowMode::default()
        .dimensions(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let window_setup = WindowSetup::default()
        .title("Tetris - Score: 0")
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
        Err(e) => eprintln!("Error occured: {}", e),
    }
}

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    Cyan,
    Yellow,
    Purple,
    Green,
    Red,
    Blue,
    Orange,
    Empty,
}

impl TileType {
    fn draw(self, ctx: &mut Context, res: &Resources, pos: Point2<f32>) -> GameResult<()> {
        if pos.y < 2.0 {
            return Ok(());
        }

        let x = FIELD_OFFSET + pos.x * TILE_SIZE;
        let y = FIELD_OFFSET + (pos.y - 2.0) * TILE_SIZE;

        let rect = Rect::new(((self as i32) as f32) * 0.125, 0.0, 0.125, 1.0);
        let draw_param = DrawParam::default()
            .src(rect)
            .dest(Point2 { x, y });
        
        graphics::draw(ctx, &res.image, draw_param)
    }
}

enum Orientation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Orientation {
    fn rotate(&mut self, clockwise: bool) {
        *self = match self {
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
    fn new_random() -> Tetrimino {
        match rand::thread_rng().gen_range(0, 7) {
            0 => Tetrimino::new_i(),
            1 => Tetrimino::new_o(),
            2 => Tetrimino::new_t(),
            3 => Tetrimino::new_s(),
            4 => Tetrimino::new_z(),
            5 => Tetrimino::new_j(),
            _ => Tetrimino::new_l(),
        }
    }

    fn new_i() -> Tetrimino {
        Tetrimino {
            tile_type: TileType::Cyan,
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
            tile_type: TileType::Yellow,
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
            tile_type: TileType::Purple,
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
            tile_type: TileType::Green,
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
            tile_type: TileType::Red,
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
            tile_type: TileType::Blue,
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
            tile_type: TileType::Orange,
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
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let state = GameState {
            res: Resources::new(ctx)?,
            
            timer: Duration::default(),
            instance: GameInstance::new(),
        };

        Ok(state)
    }
}

struct GameInstance {
    current: Tetrimino,
    map: [TileType; FIELD_TILE_COUNT],
    score: usize,
}

impl GameInstance {
    pub fn new() -> GameInstance {
        GameInstance {
            current: Tetrimino::new_random(),
            map: [TileType::Empty; FIELD_TILE_COUNT],
            score: 0,
        }
    }

    fn rotate(&mut self, clockwise: bool) {
        // clockwise rotation
        // (1,0) -> (0,1)
        // (0,1) -> (-1,0)
        // A x = x'
        // A = [ [0,-1], [1,0]]

        let mut new_tiles = [Point2 { x: 0.0, y: 0.0 }; 4];

        for i in 0..4 {
            let mut x = self.current.tiles[i].y;
            let mut y = -self.current.tiles[i].x;

            if clockwise {
                x = -x;
                y = -y;
            }

            new_tiles[i] = Point2 { x, y };
        }

        if self.collision(&self.current.pos, &new_tiles, 0.0, 0.0) {
            return;
        }

        self.current.tiles = new_tiles;
        self.current.orientation.rotate(clockwise);
    }

    fn mov(&mut self, x_off: f32, y_off: f32) -> bool {
        if self.collision(&self.current.pos, &self.current.tiles, x_off, y_off) {
            return true;
        }
        
        self.current.pos.x += x_off;
        self.current.pos.y += y_off;

        false
    }

    fn left(&mut self) {
        self.mov(-1.0, 0.0);
    }

    fn right(&mut self) {
        self.mov(1.0, 0.0);
    }

    fn drop(&mut self) -> bool {
        if !self.mov(0.0, 1.0) {
            return true;
        }

        for &pos in self.current.tiles.iter() {
            let x = (self.current.pos.x + pos.x).round() as usize;
            let y = (self.current.pos.y + pos.y).round() as usize;

            self.map[FIELD_WIDTH * y + x] = self.current.tile_type;
        }

        self.score();

        let new_tet = Tetrimino::new_random();
        
        if self.collision(&new_tet.pos, &new_tet.tiles, 0.0, 0.0) {
            // TODO
            panic!("YOU FOOL!");
        }

        self.current = new_tet;
        false
    }

    fn drop_hard(&mut self) {
        while self.drop() {}
    }

    fn score(&mut self) {
        let mut count = 0;
        let mut lines = [0; 5];

        for y in (0..FIELD_HEIGHT).rev() {
            let mut full = true;

            for x in 0..FIELD_WIDTH {
                if self.map[FIELD_WIDTH * y + x] == TileType::Empty {
                    full = false;
                    break;
                }
            }

            if full {
                lines[count] = y;
                count += 1;
            }
        }

        for i in 0..count {
            for y in (lines[i + 1]..lines[i]).rev() {
                for x in 0..FIELD_WIDTH {
                    self.map[FIELD_WIDTH * (y + i + 1) + x] = self.map[FIELD_WIDTH * y + x];
                }
            }
        }

        for y in 0..count {
            for x in 0..FIELD_WIDTH {
                self.map[FIELD_WIDTH * y + x] = TileType::Empty;
            }
        }

        if count == 0 {
            return;
        }

        self.score += (1 << (count - 1)) * 100;
    }

    fn collision(&self, pos: &Point2<f32>, tiles: &[Point2<f32>; 4], x_off: f32, y_off: f32) -> bool {
        for tile in tiles {
            let x = (pos.x + tile.x + x_off).round() as usize;
            let y = (pos.y + tile.y + y_off).round() as usize;
    
            // (x < 0 || y < 0) is tested within next check because of usize wrap-around

            if x >= FIELD_WIDTH || y >= FIELD_HEIGHT {
                return true;
            }

            if self.map[y * FIELD_WIDTH + x] != TileType::Empty {
                return true;
            }
        }

        false
    }

    fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                let pos: Point2<f32>  = Point2 { x: x as f32, y: y as f32 };
                self.map[y * FIELD_WIDTH + x].draw(ctx, res, pos)?;
            }
        }

        self.current.draw(ctx, res)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.timer += timer::delta(ctx);

        while self.timer.as_millis() >= 500 {
            self.instance.drop();
            self.timer -= Duration::from_millis(500);
        }

        timer::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.75, 0.75, 0.75, 1.0));

        self.instance.draw(ctx, &self.res)?;
        let title = String::from("Tetris - Score: ") + &self.instance.score.to_string();
        graphics::window(ctx).set_title(&title);

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool, ) {
        if !repeat {
            match keycode {
                KeyCode::Up => self.instance.rotate(true),
                KeyCode::Left => self.instance.left(),
                KeyCode::Right => self.instance.right(),
                KeyCode::Down => self.instance.drop_hard(),
                _ => (),
            }
        }
    }
}
