use ggez::{event, GameResult, graphics};

mod game;
use game::{core::MainState, constants};

pub fn main() -> GameResult{
    let cb = ggez::ContextBuilder::new("Breakout", "Nathan Barnett")
        .window_setup(ggez::conf::WindowSetup::default().title("Breakout!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 800.0));

    let (ctx, events_loop) = cb.build()?;

    let state = MainState::new(&ctx);
    event::run(ctx, events_loop, state)
}