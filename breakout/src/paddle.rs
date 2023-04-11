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
    screen_width: f32, 
    screen_height: f32,
    player_1_pos: na::Point2<f32>,
}

impl Paddle {
    pub fn new(screen_width: f32, screen_height: f32) -> Paddle {
        Paddle { 
            screen_width: screen_width, 
            screen_height: screen_height,
            player_1_pos : na::Point2::new(screen_width * 0.5, screen_height - RACKET_HEIGHT)
        }
    }
 

    fn clamp(&mut self, low: f32, high: f32) {
        if self.player_1_pos.x < low {
            self.player_1_pos.x = low;
        } else if self.player_1_pos.x > high {
            self.player_1_pos.x = high;
        }
    }

    fn move_paddle(&mut self, keycode: event::KeyCode, x_dir: f32, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        if keyboard::is_key_pressed(ctx, keycode) {
            self.player_1_pos.x -= x_dir * PLAYER_SPEED * dt;
        }

        self.clamp(RACKET_WIDTH_HALF, self.screen_width - RACKET_WIDTH_HALF);
    }

    pub fn get_player_dimensions(&self) -> (f32, f32){
        return (RACKET_WIDTH, RACKET_HEIGHT)
    }

    pub fn get_player_dimensions_half(&self) -> (f32, f32){
        return (RACKET_WIDTH * 0.5, RACKET_HEIGHT * 0.5)
    }

    pub fn get_player_pos(&self) -> na::Point2<f32> {
        return self.player_1_pos;
    }
    
    pub fn update(&mut self, ctx: &mut Context) {
        self.move_paddle(event::KeyCode::A, 1.0, ctx);
        self.move_paddle(event::KeyCode::D, -1.0, ctx);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut draw_param = graphics::DrawParam::default();
        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, graphics::WHITE)?;
        
        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;
        Ok(())
    }
}




