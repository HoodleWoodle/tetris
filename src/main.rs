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

const COLOR_EMPTY: Color = graphics::WHITE;

fn main() {
    //TODO:
    //- Sound
    //- Icon
    //- Score berechnen und anzeigen (siehe score())
    //- Refactoring
    //- Spielerlebnis: (z.B.: hard_drop und line removal schoener darstallen)
    //- Game over + Retry Screen
    //- Schoenere Tiles

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

impl Default for Point {
    fn default() -> Point {
        Point::new(0,0)
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
    score: usize,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        GameState {
            current: Tetrimino::new_random(),
            map: [COLOR_EMPTY; FIELD_TILE_COUNT],
            timer: Duration::default(),
            score: 0,
        }
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
	self.mov(-1,0);
    }

    fn right(&mut self) {
        self.mov(1,0);
    }

    fn drop_hard(&mut self) {
        while self.drop() {}
    }

    fn drop(&mut self) -> bool {
        if !self.mov(0,1) {
            return true;
        }

        // score - line removal
        // level
        for &pos in self.current.tiles.iter() {
            self.map[FIELD_WIDTH * pos.y + pos.x] = self.current.color;
        }

	self.score();

        let new_tet = Tetrimino::new_random();

	for &pos in new_tet.tiles.iter() {
    		if self.collision(pos.x as isize, pos.y as isize) {
        		//TODO
        		panic!("YOU FOOL!");
    		}
        }

        self.current = new_tet;
        false
    }

    fn score(&mut self) {
        let mut count = 0;
        let mut lines = [0; 5];

        for y in (0..FIELD_HEIGHT).rev() {
            let mut full = true;
            for x in 0..FIELD_WIDTH {
                if self.map[FIELD_WIDTH * y + x] == COLOR_EMPTY {
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
            for y in (lines[i+1]..lines[i]).rev() {
                for x in 0..FIELD_WIDTH {
                    self.map[FIELD_WIDTH * (y + i + 1) + x] = self.map[FIELD_WIDTH * y + x];
                }
            }
        }

        for y in 0..count {
            for x in 0..FIELD_WIDTH {
                self.map[FIELD_WIDTH * y + x] = COLOR_EMPTY;
            }
        }
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

		if self.map[y * FIELD_WIDTH + x] != COLOR_EMPTY {
    			return true;
		}

		false
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
