use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{Rect, Mesh, DrawMode, DrawParam, Color};
use ggez::event::{self, EventHandler};

struct Dimension {
    width: usize,
    height: usize,
}

const FIELD: Dimension = Dimension { width: 10, height: 16 };
const TILE_SIZE: usize = 32;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Game", "HoodleWoodle")
	.build()
	.expect("aieee, could not create ggez context!");

    let mut game = Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
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
        Point { x, y }
    }
}

struct Tetrimino {
    tiles: [Point; 4],
    center: usize,
    color: Color,
}

impl Tetrimino {
    fn new(tiles: &[Point; 4], center: usize, color: &Color) -> Tetrimino {
        Tetrimino { tiles: *tiles, center, color: *color }
    }
}

struct Game {
	current: Tetrimino,
	map: [Color; FIELD.width * FIELD.height],
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let current = Tetrimino::new(&[ Point::new(0,0), Point::new(1,0), Point::new(2,0), Point::new(3,0)], 1, &Color::new(0.5,0.5,0.0,1.0));
        Game {
            current,
            map: [graphics::BLACK; FIELD.width * FIELD.height],
        }
    }
}

fn draw_tile(ctx: &mut Context, pos: &Point, col: &Color) -> GameResult<()> {
	let x = (pos.x * TILE_SIZE) as f32;
	let y = (pos.y * TILE_SIZE) as f32;

	let rect = Rect::new(x, y, TILE_SIZE as f32, TILE_SIZE as f32);
	let rect = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, *col)?;

	graphics::draw(ctx, &rect, DrawParam::default())
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
       	//TODO
       	Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
	graphics::clear(ctx, graphics::WHITE);

	for y in 0..FIELD.height {
    		for x in 0..FIELD.width {
        		let pos = Point { x, y };
        		draw_tile(ctx, &pos, &self.map[y * FIELD.width + x])?;
    		}
    	}

	graphics::present(ctx)
    }
}
