use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Mesh, Rect};
use ggez::input::keyboard;
use ggez::{self, graphics};
use ggez::{Context, GameResult};

pub struct Paddle {
    pub paddle_pos: ggez::glam::Vec2,
    pub paddle_size: Vec2,
    pub paddle_vel: f32,
    paddle_speed: f32,
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle {
            paddle_pos: Vec2::new(0.0, 0.0),
            paddle_size: Vec2::new(0.0, 0.0),
            paddle_vel: 0.0,
            paddle_speed: 0.0,
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
            self.paddle_pos.x -= x_dir * self.paddle_speed * dt;
            self.paddle_vel = x_dir;
        }

        self.clamp(0.0, ctx.gfx.drawable_size().0 - self.paddle_size.x);
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.paddle_vel = 0.0;
        self.move_paddle(keyboard::KeyCode::A, 1.0, ctx);
        self.move_paddle(keyboard::KeyCode::D, -1.0, ctx);
    }

    pub fn update_scale(&mut self, screen_size: (f32, f32)) {
        self.update_paddle_size(screen_size);
        self.update_paddle_pos(screen_size);
        self.update_paddle_speed(screen_size);
    }

    fn update_paddle_size(&mut self, screen_size: (f32, f32)) {
        self.paddle_size = Vec2::new(screen_size.0 / 8.0, screen_size.1 / 40.0);
    }

    fn update_paddle_speed(&mut self, screen_size: (f32, f32)) {
        self.paddle_speed = screen_size.0 - 200.0;
    }

    fn update_paddle_pos(&mut self, screen_size: (f32, f32)) {
        if self.paddle_pos.x == 0.0 {
            self.paddle_pos = Vec2::new(
                screen_size.0 * 0.5,
                screen_size.1 - (self.paddle_size.y * 2.0),
            )
        } else {
            self.paddle_pos = Vec2::new(
                self.paddle_pos.x,
                screen_size.1 - (self.paddle_size.y * 2.0),
            )
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &Context) -> GameResult {
        let racket_rect = Rect::new(0.0, 0.0, self.paddle_size.x, self.paddle_size.y);

        let racket_mesh =
            Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, Color::WHITE)
                .unwrap();

        canvas.draw(&racket_mesh, self.paddle_pos);
        Ok(())
    }
}
