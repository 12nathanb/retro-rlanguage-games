use ggez::nalgebra::{self as na};
use ggez::{self, graphics};
use ggez::{Context, GameResult};
use ggez::event;
use rand::{thread_rng, Rng};
use ggez::input::keyboard;

use crate::paddle::Paddle;

const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const BALL_SPEED: f32 = 300.0;

pub struct Ball {
    ball_pos: na::Point2<f32>,
    screen_size: (f32, f32),
    ball_vel: na::Vector2<f32>,
    fire_ball: bool
}

impl Ball {
    pub fn new(ball_pos: na::Point2<f32>, screen_size: (f32, f32)) -> Ball {
        Ball {
            ball_pos: ball_pos,
            screen_size: screen_size,
            ball_vel: na::Vector2::new(0.0, 0.0),
            fire_ball: false
        }
    }

    pub fn update(&mut self, ctx: &mut Context, paddle: &Paddle) -> GameResult{
        

        if keyboard::is_key_pressed(ctx, event::KeyCode::Space) && self.fire_ball == false {
            self.fire_ball = true;
            self.randomise_vec(BALL_SPEED, BALL_SPEED);
        }

        

        if self.ball_pos.y > self.screen_size.1 { 
            self.fire_ball = false;
            self.randomise_vec(BALL_SPEED, BALL_SPEED);
        }

        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.x < BALL_SIZE_HALF {
            self.ball_pos.x = BALL_SIZE_HALF;
            self.ball_vel.x = self.ball_vel.x.abs();
        }  else if self.ball_pos.x > self.screen_size.0 - BALL_SIZE_HALF {
            self.ball_pos.x = self.screen_size.0 - BALL_SIZE_HALF;
            self.ball_vel.x = - self.ball_vel.x.abs();
        }

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, graphics::WHITE)?;
        
        let mut draw_param = graphics::DrawParam::default();

        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;
        Ok(())
    }

    fn randomise_vec(&mut self, x: f32, y: f32) {
        let mut rng = thread_rng();
        self.ball_vel.x = match rng.gen_bool(0.5) {
            true => x,
            false => -x
        };
    
        self.ball_vel.y = match rng.gen_bool(0.5) {
            true => y,
            false => -y
        };
    }

    pub fn get_fire_ball(&self) -> bool {
        return self.fire_ball;
    }

    pub fn get_ball_vel(&self) -> na::Vector2<f32> {
        return self.ball_vel;
    }

    pub fn launch_ball(&mut self, direction: na::Vector2<f32>) {
        self.ball_pos += direction;
    }

    pub fn reverse_velocity(&mut self) {
        self.ball_vel.y = - self.ball_vel.y.abs();
    }

    pub fn get_pos(&self) -> na::Point2<f32> {
        return self.ball_pos;
    }

    pub fn intersects_player(&self, paddle: &Paddle) -> bool {
        return self.ball_pos.x - BALL_SIZE_HALF < paddle.get_player_pos().x + paddle.get_player_dimensions_half().0
            && self.ball_pos.x + BALL_SIZE_HALF > paddle.get_player_pos().x - paddle.get_player_dimensions_half().0
            && self.ball_pos.y - BALL_SIZE_HALF < paddle.get_player_pos().y + paddle.get_player_dimensions_half().1
            && self.ball_pos.y + BALL_SIZE_HALF > paddle.get_player_pos().y - paddle.get_player_dimensions_half().1;
    }

    pub fn reset_ball(&mut self, x: f32, y: f32) {
        self.ball_pos.x = x;
        self.ball_pos.y = y;
    }
}