//#![windows_subsystem = "windows"]

use ggez::{
    conf::{WindowMode, WindowSetup},
    event,
    ContextBuilder,
};
use std::{
    process,
    path::Path,
    fs::File,
    env,
    path,
};

mod tetrimino;
mod settings;
mod random;
mod state;

use state::StateHandler;

fn main() {
    // ORDER:
    // - GameState
    //      - game over [gray screenshot]
    //      - count down
    // - Menu (show: start, leaderboard OR ready toggle)
    // - Leaderboard

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
    // OPTIONAL:
    // - spawn window centered
    // - uncomment line one of main.rs
    // - screenshot
    // - shadow piece
    // - hold piece
    // - font size, color setting (ingame / menus), outgray color
    // - seperate (via bounding box) value and text in ingame rendering
    // - bounding boxes to fixed positions
    // ------------------------------------------------------------------------------------------------
    // IMPROVMENTS:
    // - save generators history locally (only one local generator)
    // ------------------------------------------------------------------------------------------------
    // BUGS:
    // - disable audio player before exit
    // - quirin´s line problem
    // - window scaling bug on laptops?
    // ------------------------------------------------------------------------------------------------

    // load settings
    let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        manifest_dir + "/"
    } else {
        "".to_string()
    };

    let file = match File::open(Path::new(&(path + "resources/settings.json"))) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not load settings: {}", e);
            process::exit(1);
        }
    };

    let settings = match settings::load(file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Settings corrupted: {}", e);
            process::exit(1);
        }
    };

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

    let (width, height) = if settings.multiplayer_enabled {
        (settings.multiplayer.w, settings.multiplayer.h)
    } else {
        (settings.singleplayer.w, settings.singleplayer.h)
    };
    let window_mode = WindowMode::default()
        .dimensions(width, height);

    let (mut ctx, mut event_loop) = ctx_builder
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggez context!");

    let mut handler = match StateHandler::new(&mut ctx, settings) {
        Ok(handler) => handler,
        Err(e) => {
            eprintln!("Could not create state handler: {}", e);
            process::exit(1);
        }
    };

    // run
    match event::run(&mut ctx, &mut event_loop, &mut handler) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
