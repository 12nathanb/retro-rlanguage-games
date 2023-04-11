use ggez::{self, graphics};
use ggez::{Context, GameResult};
use ggez::event;
use ggez::nalgebra::{self as na};
mod paddle;
mod ball;

pub use paddle::Paddle;
pub use ball::Ball;

const RACKET_HEIGHT: f32 = 20.0;

struct MainState {
    player_1: Paddle,
    ball: Ball
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let(screen_w, screen_h) = graphics::drawable_size(ctx);
        let screen_w_half = screen_h * 0.5;

        MainState{
            player_1 : Paddle::new(screen_w, screen_h),
            ball: Ball::new(na::Point2::new(screen_w_half, screen_h - (RACKET_HEIGHT * 2.0)), graphics::drawable_size(ctx))
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        
        self.player_1.update(_ctx);
        
        self.ball.update(_ctx, &self.player_1);       

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, graphics::BLACK);

        self.player_1.draw(_ctx);
        self.ball.draw(_ctx);

        graphics::present(_ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Breakout", "Nathan");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "Breakout");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state);
    Ok(())
}