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
    pub fn new(_ctx: &mut Context) -> Self {
        let screen_size = graphics::drawable_size(_ctx);

        MainState{
            player_1 : Paddle::new(na::Point2::new(screen_size.0 * 0.5, screen_size.1 - RACKET_HEIGHT), graphics::drawable_size(_ctx)),
            ball: Ball::new(na::Point2::new(screen_size.0 * 0.5, screen_size.1 - (RACKET_HEIGHT * 2.0)), graphics::drawable_size(_ctx))
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let screen_size = graphics::drawable_size(_ctx);
        let dt = ggez::timer::delta(_ctx).as_secs_f32();

        self.player_1.update(_ctx);
        
        self.ball.update(_ctx, &self.player_1);       

        if self.ball.get_pos().y > screen_size.1 {
            self.ball.reset_ball(self.player_1.get_player_pos().x, self.player_1.get_player_pos().y - self.player_1.get_player_dimensions().1)
         }

         if self.ball.intersects_player(&self.player_1) {
            self.ball.reverse_velocity();
         }

         if self.ball.get_fire_ball() {
            self.ball.launch_ball(self.ball.get_ball_vel() * dt)
         } else {
             //self.ball_pos.x = paddle.get_player_pos().x;
         }

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