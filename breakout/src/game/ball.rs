use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::input::keyboard;
use ggez::{self, graphics};
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};

use super::paddle::Paddle;

pub struct Ball {
    pub ball_pos: Vec2,
    ball_vel: Vec2,
    pub ball_size: Vec2,
    pub fire_ball: bool,
    pub multiplyer: f32,
    ball_speed: f32,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            ball_pos: Vec2::new(0.0, 0.0),
            ball_vel: Vec2::new(0.0, 0.0),
            ball_size: Vec2::new(0.0, 0.0),
            fire_ball: false,
            multiplyer: 0.0,
            ball_speed: 0.0,
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        player: &Paddle,
        screen_size: (f32, f32),
    ) -> GameResult {
        if self.ball_pos.y < (self.ball_size.x * 0.5) + (screen_size.1 * 0.1) {
            self.ball_pos.y = (self.ball_size.x * 0.5) + (screen_size.1 * 0.1);
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.x < (self.ball_size.x * 0.5) {
            self.ball_pos.x = self.ball_size.x * 0.5;
            self.ball_vel.x = self.ball_vel.x.abs();
        } else if self.ball_pos.x > ctx.gfx.drawable_size().0 - (self.ball_size.x * 0.5) {
            self.ball_pos.x = ctx.gfx.drawable_size().0 - (self.ball_size.x * 0.5);
            self.ball_vel.x = -self.ball_vel.x.abs();
        }

        if self.fire_ball == false {
            self.reset_ball(
                player.paddle_pos.x + (player.paddle_size.x * 0.5),
                player.paddle_pos.y - (player.paddle_size.y * 0.5),
            );
        } else {
            self.ball_pos += self.ball_vel * ctx.time.delta().as_secs_f32();
        }

        if ctx.keyboard.is_key_pressed(keyboard::KeyCode::Space) && self.fire_ball == false {
            self.fire_ball = true;
            self.randomise_vec(self.ball_speed, self.ball_speed);
        }

        if self.intersects_player(player) {
            self.multiplyer += 1.0;
            self.player_reverse_velocity();
        }

        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &Context) -> GameResult {
        let ball_rect = graphics::Rect::new(
            -(self.ball_size.x * 0.5),
            -(self.ball_size.y * 0.5),
            self.ball_size.x,
            self.ball_size.y,
        );
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::Color::WHITE,
        )
        .unwrap();

        canvas.draw(&ball_mesh, self.ball_pos);
        Ok(())
    }

    pub fn update_ball_size(&mut self, screen_size: (f32, f32)) {
        self.ball_size = Vec2::new(screen_size.1 / 40.0, screen_size.1 / 40.0);
        self.ball_speed = screen_size.1 * 0.6;
    }

    fn randomise_vec(&mut self, x: f32, y: f32) {
        let mut rng = thread_rng();
        self.ball_vel.x = match rng.gen_bool(0.5) {
            true => x,
            false => -x,
        };

        self.ball_vel.y = match rng.gen_bool(0.5) {
            true => -y,
            false => -y,
        };
    }

    pub fn reverse_velocity(&mut self) {
        self.ball_vel.y *= -1.0;
    }

    pub fn player_reverse_velocity(&mut self) {
        self.ball_vel.y = -self.ball_vel.y.abs() + -self.multiplyer;
    }

    pub fn intersects_player(&self, paddle: &Paddle) -> bool {
        return self.ball_pos.x - (self.ball_size.x * 0.5)
            < paddle.paddle_pos.x + paddle.paddle_size.x
            && self.ball_pos.x + (self.ball_size.x * 0.5) > paddle.paddle_pos.x
            && self.ball_pos.y - (self.ball_size.x * 0.5) < paddle.paddle_pos.y
            && self.ball_pos.y + (self.ball_size.x * 0.5) > paddle.paddle_pos.y;
    }

    pub fn reset_ball(&mut self, x: f32, y: f32) {
        self.ball_pos.x = x;
        self.ball_pos.y = y;
    }
}
