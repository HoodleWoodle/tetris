use ggez::{
    Context, ContextBuilder, GameResult,
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Rect, Mesh, DrawMode, DrawParam, Color},
    timer,
};
use rand::{self, Rng};
use std::time::Duration;

const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 16;
const FIELD_TILE_COUNT: usize = FIELD_WIDTH * FIELD_HEIGHT;

const FIELD_OFFSET: usize = 1;

const TILE_SIZE: usize = 32;
const TILE_SPACING: usize = 1;

const WINDOW_WIDTH: usize = TILE_SIZE * FIELD_WIDTH + TILE_SPACING * (FIELD_WIDTH - 1) + FIELD_OFFSET * 2;
const WINDOW_HEIGHT: usize = TILE_SIZE * FIELD_HEIGHT + TILE_SPACING * (FIELD_HEIGHT - 1) + FIELD_OFFSET * 2;

fn main() {
    let window_mode = WindowMode::default()
        .dimensions(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let window_setup = WindowSetup::default()
        .title("Tetris");

    let (mut ctx, mut event_loop) = ContextBuilder::new("tetris", "author")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("Could not create ggez context!");

    let mut state = GameState::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut state) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
        }
    }
}

struct Tetrimino {
    tiles: [Point; 4],
    center: Option<usize>,
    color: Color,
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
            6 => Tetrimino::new_l(),
            _ => panic!("dead code"),
        }
    }

    fn new_i() -> Tetrimino {
        Tetrimino {
            tiles: [ Point::new(3, 0), Point::new(4, 0), Point::new(5, 0), Point::new(6, 0)],
            center: Some(1),
            color: Color::new(0.0, 0.9375, 0.9375, 1.0),
        }
    }

    fn new_o() -> Tetrimino {
        Tetrimino {
            tiles: [ Point::new(4, 0), Point::new(4, 1), Point::new(5, 0), Point::new(5, 1)],
            center: None,
            color: Color::new(0.9375, 0.9375, 0.0, 1.0),
        }
    }

    fn new_t() -> Tetrimino {
        Tetrimino {
            tiles: [ Point::new(4, 0), Point::new(5, 0), Point::new(5, 1), Point::new(6, 0)],
            center: Some(1),
            color: Color::new(0.625, 0.0, 0.9375, 1.0),
        }
    }

    fn new_s() -> Tetrimino {
        Tetrimino {
            tiles: [ Point::new(4, 1), Point::new(5, 0), Point::new(5, 1), Point::new(6, 0)],
            center: Some(2),
            color: Color::new(0.0, 0.9375, 0.0, 1.0),
        }
    }

    fn new_z() -> Tetrimino {
        Tetrimino {
            tiles: [ Point::new(4, 0), Point::new(5, 0), Point::new(5, 1), Point::new(6, 1)],
            center: Some(2),
            color: Color::new(0.9375, 0.0, 0.0, 1.0),
        }
    }

    fn new_j() -> Tetrimino {
        Tetrimino {
            tiles: [ Point::new(4, 0), Point::new(5, 0), Point::new(6, 0), Point::new(6, 1)],
            center: Some(1),
            color: Color::new(0.0, 0.0, 0.9375, 1.0),
        }
    }

    fn new_l() -> Tetrimino {
        Tetrimino {
            tiles: [ Point::new(4, 0), Point::new(4, 1), Point::new(5, 0), Point::new(6, 0)],
            center: Some(2),
            color: Color::new(0.9375, 0.625, 0.0, 1.0),
        }
    }
}

struct GameState {
	current: Tetrimino,
    map: [Color; FIELD_TILE_COUNT],
    timer: Duration,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        GameState {
            current: Tetrimino::new_random(),
            map: [graphics::WHITE; FIELD_TILE_COUNT],
            timer: Duration::default(),
        }
    }

    fn rotate(&mut self) {

    }

    fn left(&mut self) {
    }

    fn right(&mut self) {
    }

    fn drop_hard(&mut self) {

    }

    fn drop(&mut self) {

        // score - line removal
        // level
    }
}

fn draw_tile(ctx: &mut Context, pos: Point, col: Color) -> GameResult<()> {
	let x = (FIELD_OFFSET + pos.x * (TILE_SIZE + TILE_SPACING)) as f32;
	let y = (FIELD_OFFSET + pos.y * (TILE_SIZE + TILE_SPACING)) as f32;

	let rect = Rect::new(x, y, TILE_SIZE as f32, TILE_SIZE as f32);
	let rect = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, col)?;

	graphics::draw(ctx, &rect, DrawParam::default())
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.timer += timer::delta(ctx);

        while self.timer.as_millis() >= 500 {
            self.drop();
            self.timer -= Duration::from_millis(500);
        }

        timer::yield_now();
       	Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.65, 0.65, 0.65, 1.0));

        for y in 0..FIELD_HEIGHT {
    		for x in 0..FIELD_WIDTH {
        		let pos = Point { x, y };
        		draw_tile(ctx, pos, self.map[y * FIELD_WIDTH + x])?;
    		}
        }
        
        for &pos in self.current.tiles.iter() {
            draw_tile(ctx, pos, self.current.color)?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool)
    {
        if !repeat {
            match keycode {
                KeyCode::Up => self.rotate(),
                KeyCode::Left => self.left(),
                KeyCode::Right => self.right(),
                KeyCode::Down => self.drop_hard(),
                _ => (),
            }
        }
    }
}
