use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Mesh, Rect};
use ggez::input::keyboard;
use ggez::{self, graphics};
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};

use crate::game::constants::*;

pub struct Ball {
    pub ball_pos: Vec2,
    ball_vel: Vec2,
    fire_ball: bool,
    ball_mesh: Mesh,
    pub ball_rect: Rect,
    multiplyer: f32,
}

impl Ball {
    pub fn new(ctx: &Context, ball_pos: Vec2) -> Ball {
        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::Color::WHITE,
        )
        .unwrap();

        Ball {
            ball_pos: ball_pos,
            ball_vel: Vec2::new(0.0, 0.0),
            fire_ball: false,
            ball_mesh: ball_mesh,
            ball_rect: ball_rect,
            multiplyer: 0.0,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, player_pos: Vec2, player_vel: f32) -> GameResult {
        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.x < BALL_SIZE_HALF {
            self.ball_pos.x = BALL_SIZE_HALF;
            self.ball_vel.x = self.ball_vel.x.abs();
        } else if self.ball_pos.x > ctx.gfx.drawable_size().0 - BALL_SIZE_HALF {
            self.ball_pos.x = ctx.gfx.drawable_size().0 - BALL_SIZE_HALF;
            self.ball_vel.x = -self.ball_vel.x.abs();
        }

        if self.fire_ball == false {
            self.reset_ball(
                player_pos.x + RACKET_WIDTH_HALF,
                player_pos.y - RACKET_HEIGHT_HALF,
            );
        } else {
            //self.launch_ball(self.ball_vel);
            self.ball_pos += self.ball_vel * ctx.time.delta().as_secs_f32();
        }

        if ctx.keyboard.is_key_pressed(keyboard::KeyCode::Space) && self.fire_ball == false {
            self.fire_ball = true;
            self.randomise_vec(BALL_SPEED, BALL_SPEED);
        }

        if self.intersects_player(player_pos) {
            //self.randomise_vec(BALL_SPEED, BALL_SPEED);
            self.multiplyer += 10.0;
            self.player_reverse_velocity(player_vel);
        }

        if self.ball_pos.y > ctx.gfx.drawable_size().1 {
            self.fire_ball = false;
            self.multiplyer = 0.0;
        }

        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas) -> GameResult {
        canvas.draw(&self.ball_mesh, self.ball_pos);
        Ok(())
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

    pub fn player_reverse_velocity(&mut self, player_vel: f32) {
        self.ball_vel.y = -self.ball_vel.y.abs();
        self.ball_vel.x *= -player_vel.abs();
    }

    pub fn intersects_player(&self, paddle: Vec2) -> bool {
        return self.ball_pos.x - BALL_SIZE_HALF < paddle.x + RACKET_WIDTH
            && self.ball_pos.x + BALL_SIZE_HALF > paddle.x
            && self.ball_pos.y - BALL_SIZE_HALF < paddle.y
            && self.ball_pos.y + BALL_SIZE_HALF > paddle.y;
    }

    pub fn reset_ball(&mut self, x: f32, y: f32) {
        self.ball_pos.x = x;
        self.ball_pos.y = y;
    }
}
