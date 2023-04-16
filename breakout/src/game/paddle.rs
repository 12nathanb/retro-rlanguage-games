use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Rect, Mesh};
use ggez::{self, graphics};
use ggez::{Context, GameResult};
use ggez::input::keyboard;

use crate::game::{constants::*};


pub struct Paddle {
    pub paddle_pos: ggez::glam::Vec2,
    pub paddle_mesh: Mesh
}

impl Paddle {
    pub fn new(ctx: &Context, paddle_pos: ggez::glam::Vec2) -> Paddle {
        print!("{}\n", ctx.gfx.drawable_size().0);
        let racket_rect = Rect::new(0.0, 0.0, RACKET_WIDTH, RACKET_HEIGHT);

        let racket_mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            Color::WHITE
        ).unwrap();
        
        Paddle { 
            paddle_pos : paddle_pos,
            paddle_mesh: racket_mesh
        }
    }

    fn clamp(&mut self, low: f32, high: f32) {
        if self.paddle_pos.x < low {
            self.paddle_pos.x = low;
        } else if self.paddle_pos.x > high {
            self.paddle_pos.x = high;
        }
    }

    fn move_paddle(&mut self, keycode: keyboard::KeyCode, x_dir: f32, ctx: &mut Context) {
        let dt = ctx.time.delta().as_secs_f32();
        if ctx.keyboard.is_key_pressed(keycode) {
            self.paddle_pos.x -= x_dir * PLAYER_SPEED * dt;
            print!("X: {} Y: {}\n", self.paddle_pos.x, self.paddle_pos.y);
        }

        self.clamp(0.0, ctx.gfx.drawable_size().0 - RACKET_WIDTH);
    }


    pub fn update(&mut self, ctx: &mut Context) {
        self.move_paddle(keyboard::KeyCode::A, 1.0, ctx);
        self.move_paddle(keyboard::KeyCode::D, -1.0, ctx);
    }

    pub fn draw(&self, canvas: &mut Canvas) -> GameResult{
        canvas.draw(&self.paddle_mesh, Vec2::new(self.paddle_pos.x, self.paddle_pos.y));
        Ok(())
    }
}