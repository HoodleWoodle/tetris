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
const FIELD_HEIGHT: usize = 16;
const FIELD_TILE_COUNT: usize = FIELD_WIDTH * FIELD_HEIGHT;
const FIELD_OFFSET: usize = 2;

const TILE_SIZE: usize = 32;
const TILE_SPACING: usize = 0;

const WINDOW_WIDTH: usize = TILE_SIZE * FIELD_WIDTH + TILE_SPACING * (FIELD_WIDTH - 1) + FIELD_OFFSET * 2;
const WINDOW_HEIGHT: usize = TILE_SIZE * FIELD_HEIGHT + TILE_SPACING * (FIELD_HEIGHT - 1) + FIELD_OFFSET * 2;

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

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl Default for Point {
    fn default() -> Point {
        Point::new(0, 0)
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

struct Tetrimino {
    tiles: [Point; 4],
    center: Option<usize>,
    tile_type: TileType,
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
            tiles: [
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(5, 0),
                Point::new(6, 0),
            ],
            center: Some(1),
            tile_type: TileType::Cyan,
        }
    }

    fn new_o() -> Tetrimino {
        Tetrimino {
            tiles: [
                Point::new(4, 0),
                Point::new(4, 1),
                Point::new(5, 0),
                Point::new(5, 1),
            ],
            center: None,
            tile_type: TileType::Yellow,
        }
    }

    fn new_t() -> Tetrimino {
        Tetrimino {
            tiles: [
                Point::new(4, 0),
                Point::new(5, 0),
                Point::new(5, 1),
                Point::new(6, 0),
            ],
            center: Some(1),
            tile_type: TileType::Purple,
        }
    }

    fn new_s() -> Tetrimino {
        Tetrimino {
            tiles: [
                Point::new(4, 1),
                Point::new(5, 0),
                Point::new(5, 1),
                Point::new(6, 0),
            ],
            center: Some(2),
            tile_type: TileType::Green,
        }
    }

    fn new_z() -> Tetrimino {
        Tetrimino {
            tiles: [
                Point::new(4, 0),
                Point::new(5, 0),
                Point::new(5, 1),
                Point::new(6, 1),
            ],
            center: Some(2),
            tile_type: TileType::Red,
        }
    }

    fn new_j() -> Tetrimino {
        Tetrimino {
            tiles: [
                Point::new(4, 0),
                Point::new(5, 0),
                Point::new(6, 0),
                Point::new(6, 1),
            ],
            center: Some(1),
            tile_type: TileType::Blue,
        }
    }

    fn new_l() -> Tetrimino {
        Tetrimino {
            tiles: [
                Point::new(4, 0),
                Point::new(4, 1),
                Point::new(5, 0),
                Point::new(6, 0),
            ],
            center: Some(2),
            tile_type: TileType::Orange,
        }
    }
}

struct GameState {
    _sound: Source,
    image: Image,

    current: Tetrimino,
    map: [TileType; FIELD_TILE_COUNT],
    timer: Duration,
    score: usize,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut _sound = Source::new(ctx, Path::new("/sound.ogg"))?;
        _sound.set_repeat(true);
        _sound.set_volume(0.025);
        _sound.play()?;

        let image = Image::new(ctx, Path::new("/tiles.png"))?;

        let state = GameState {
            _sound,
            image,

            current: Tetrimino::new_random(),
            map: [TileType::Empty; FIELD_TILE_COUNT],
            timer: Duration::default(),
            score: 0,
        };

        Ok(state)
    }

    fn rotate(&mut self) {
        let center = match self.current.center {
            Some(i) => self.current.tiles[i],
            None => return,
        };

        // (1,0) -> (0,1)
        // (0,1) -> (-1,0)
        // A x = x'
        // A = [ [0,-1], [1,0]]

        let mut new_tiles = [Point::default(); 4];

        for i in 0..4 {
            let x_old = self.current.tiles[i].x as isize;
            let y_old = self.current.tiles[i].y as isize;

            let cx = center.x as isize;
            let cy = center.y as isize;

            let x = cx - (y_old - cy);
            let y = cy + (x_old - cx);

            if self.collision(x, y) {
                return;
            }

            new_tiles[i] = Point::new(x as usize, y as usize);
        }

        self.current.tiles = new_tiles;
    }

    fn mov(&mut self, x_off: isize, y_off: isize) -> bool {
        let mut new_tiles = [Point::default(); 4];

        for i in 0..4 {
            let x = self.current.tiles[i].x as isize + x_off;
            let y = self.current.tiles[i].y as isize + y_off;

            if self.collision(x, y) {
                return true;
            }

            new_tiles[i] = Point::new(x as usize, y as usize);
        }

        self.current.tiles = new_tiles;
        false
    }

    fn left(&mut self) {
        self.mov(-1, 0);
    }

    fn right(&mut self) {
        self.mov(1, 0);
    }

    fn drop_hard(&mut self, ctx: &mut Context) {
        while self.drop(ctx) {}
    }

    fn drop(&mut self, ctx: &mut Context) -> bool {
        if !self.mov(0, 1) {
            return true;
        }

        for &pos in self.current.tiles.iter() {
            self.map[FIELD_WIDTH * pos.y + pos.x] = self.current.tile_type;
        }

        self.score(ctx);

        let new_tet = Tetrimino::new_random();

        for &pos in new_tet.tiles.iter() {
            if self.collision(pos.x as isize, pos.y as isize) {
                // TODO
                panic!("YOU FOOL!");
            }
        }

        self.current = new_tet;
        false
    }

    fn score(&mut self, ctx: &mut Context) {
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

        let title = String::from("Tetris - Score: ") + &self.score.to_string();
        graphics::window(ctx).set_title(&title);
    }

    fn collision(&mut self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return true;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= FIELD_WIDTH || y >= FIELD_HEIGHT {
            return true;
        }

        if self.map[y * FIELD_WIDTH + x] != TileType::Empty {
            return true;
        }

        false
    }

    fn draw_tile(&self, ctx: &mut Context, pos: Point, tile_type: TileType) -> GameResult<()> {
        let x = (FIELD_OFFSET + pos.x * (TILE_SIZE + TILE_SPACING)) as f32;
        let y = (FIELD_OFFSET + pos.y * (TILE_SIZE + TILE_SPACING)) as f32;

        let rect = Rect::new(((tile_type as i32) as f32) * 0.125, 0.0, 0.125, 1.0);
        let draw_param = DrawParam::default()
            .src(rect)
            .dest(Point2 { x, y });
        
        graphics::draw(ctx, &self.image, draw_param)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.timer += timer::delta(ctx);

        while self.timer.as_millis() >= 500 {
            self.drop(ctx);
            self.timer -= Duration::from_millis(500);
        }

        timer::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.75, 0.75, 0.75, 1.0));

        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                let pos = Point { x, y };
                self.draw_tile(ctx, pos, self.map[y * FIELD_WIDTH + x])?;
            }
        }

        for &pos in self.current.tiles.iter() {
            self.draw_tile(ctx, pos, self.current.tile_type)?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool, ) {
        if !repeat {
            match keycode {
                KeyCode::Up => self.rotate(),
                KeyCode::Left => self.left(),
                KeyCode::Right => self.right(),
                KeyCode::Down => self.drop_hard(ctx),
                _ => (),
            }
        }
    }
}
