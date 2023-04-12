use ggez::nalgebra::{self as na};
use ggez::{self, graphics};
use ggez::{Context, GameResult};
use ggez::input::keyboard;
use ggez::event;

const PLAYER_SPEED: f32 = 600.0;
const RACKET_HEIGHT: f32 = 20.0;
const RACKET_WIDTH: f32 = 100.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;

pub struct Paddle {
    screen_size: (f32, f32),
    paddle_pos: na::Point2<f32>,
}

impl Paddle {
    pub fn new(paddle_pos: na::Point2<f32>, screen_size: (f32, f32)) -> Paddle {
        Paddle { 
            screen_size: screen_size,
            paddle_pos : paddle_pos
        }
    }
 

    fn clamp(&mut self, low: f32, high: f32) {
        if self.paddle_pos.x < low {
            self.paddle_pos.x = low;
        } else if self.paddle_pos.x > high {
            self.paddle_pos.x = high;
        }
    }

    fn move_paddle(&mut self, keycode: event::KeyCode, x_dir: f32, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        if keyboard::is_key_pressed(ctx, keycode) {
            self.paddle_pos.x -= x_dir * PLAYER_SPEED * dt;
        }

        self.clamp(RACKET_WIDTH_HALF, self.screen_size.0 - RACKET_WIDTH_HALF);
    }

    pub fn get_player_dimensions(&self) -> (f32, f32){
        return (RACKET_WIDTH, RACKET_HEIGHT)
    }

    pub fn get_player_dimensions_half(&self) -> (f32, f32){
        return (RACKET_WIDTH * 0.5, RACKET_HEIGHT * 0.5)
    }

    pub fn get_player_pos(&self) -> na::Point2<f32> {
        return self.paddle_pos;
    }
    
    pub fn update(&mut self, ctx: &mut Context) {
        self.move_paddle(event::KeyCode::A, 1.0, ctx);
        self.move_paddle(event::KeyCode::D, -1.0, ctx);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut draw_param = graphics::DrawParam::default();
        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, graphics::WHITE)?;
        
        draw_param.dest = self.paddle_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;
        Ok(())
    }
}




