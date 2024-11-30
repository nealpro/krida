mod game;

use ggez::conf;
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use std::env;
use std::path;

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let (grid_width, grid_height) = (
        (game::GRID_WIDTH as f32) * game::CELL_SIZE,
        (game::GRID_HEIGHT as f32) * game::CELL_SIZE,
    );
    let cb = ContextBuilder::new("krida", "nealpro")
        .window_setup(conf::WindowSetup::default().title("Krida - Game of Life"))
        .window_mode(conf::WindowMode::default().dimensions(grid_width, grid_height))
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = game::MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
