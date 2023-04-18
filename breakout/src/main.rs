use game::core::MainState;
use ggez::{event, GameResult};

mod game;
use std::{env, path};

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::current_dir() {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");

        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("Breakout", "Nathan Barnett")
        .window_setup(ggez::conf::WindowSetup::default().title("Breakout!"))
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let state = MainState::new(&mut ctx);
    event::run(ctx, events_loop, state)
}
