//#![windows_subsystem = "windows"]

#[macro_use]
extern crate glium;

use crate::ggwp::{
    conf::{WindowMode, WindowSetup},
    event,
    ContextBuilder,
};
use std::{
    path::Path,
    fs::File,
    env,
    path,
};

mod ggwp;
mod tetrimino;
mod settings;
mod random;
mod map;
pub mod state;

use state::StateHandler;

fn main() {
    // TODO:
    // - popup for each game instance
    // - menu (show: start)
    
    // ------------------------------------------------------------------------------------------------
    // MULTIPLAYER
    // - both player are added to the local leaderboard
    // ------------------------------------------------------------------------------------------------
    // NETWORKING:
    // - settings handshake
    // - initial command (seed, nicknames)
    // - ready command
    // - ...
    // ------------------------------------------------------------------------------------------------

    // ------------------------------------------------------------------------------------------------
    // OPTIONAL FEATURES:
    // - menu (show: leaderboard OR ready toggle)
    // - leaderboard
    // - sound effects
    // - shadow piece
    // - hold piece
    // ------------------------------------------------------------------------------------------------
    // OPTIONAL IMPROVMENTS:
    // - save generators history locally (only one local generator)
    // ------------------------------------------------------------------------------------------------
    // FINAL:
    // - uncomment line one of main.rs
    // ------------------------------------------------------------------------------------------------

    // load settings
    let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        manifest_dir + "/"
    } else {
        "".to_string()
    };

    let file = File::open(Path::new(&(path + "resources/settings.json")))
        .expect("Could not load settings");
    let settings = settings::load(file)
        .expect("Settings corrupted");

    // build context
    let mut ctx_builder = ContextBuilder::new("tetris", "");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx_builder = ctx_builder.add_resource_path(path);
    }

    let window_setup = WindowSetup::default()
        .title("Tetris")
        .icon("/icon.png");

    let background = settings.background();
    let window_mode = WindowMode::default()
        .dimensions(background.w, background.h);

    let (mut ctx, mut event_loop) = ctx_builder
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggwp context!");

    let mut handler = StateHandler::new(&mut ctx, settings)
        .expect("Could not create state handler!");

    // run
    event::run(&mut ctx, &mut event_loop, &mut handler)
        .expect("Error occurred");
   
    println!("Exited cleanly.");
}
