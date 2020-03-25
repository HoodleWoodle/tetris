use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::*;
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Game", "HoodleWoodle")
		.build()
		.expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut game = Game::new(&mut ctx);

    // Run!
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
		let rect = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, BLACK).unwrap();
		
        Game {
		    rect: rect
		}
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::WHITE);
		
		graphics::draw(ctx, &self.rect, DrawParam::new())?;
		
		graphics::present(ctx)
    }
}