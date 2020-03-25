use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{Rect, Mesh, DrawMode, DrawParam};
use ggez::event::{self, EventHandler};

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

struct Game {
	rect: Mesh,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
		let rect = Rect::new(20.0, 30.0, 50.0, 70.0);
		let rect = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, graphics::BLACK).unwrap();
		
        Game {
		    rect: rect
		}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::WHITE);
		
		graphics::draw(ctx, &self.rect, DrawParam::new())?;
		
		graphics::present(ctx)
    }
}