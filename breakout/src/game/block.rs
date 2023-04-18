use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
    Context, GameResult,
};

use super::constants::BLOCK_AMOUNT_F32;

#[derive(Default)]
pub struct Block {
    pub block_size: Vec2,
    pub block_pos: Vec2,
    pub block_coord: (i32, i32),
    selected_color: (u8, u8, u8),
}

impl Block {
    pub fn new(color: (u8, u8, u8), block_coord: (i32, i32)) -> Block {
        Block {
            block_size: Vec2::new(0.0, 0.0),
            block_pos: Vec2::new(0.0, 0.0),
            block_coord: block_coord,
            selected_color: color,
        }
    }

    pub fn update_scale(&mut self, screen_size: (f32, f32)) -> Vec2 {
        self.block_size = self.get_block_size(screen_size);
        self.block_pos = self.get_block_pos(screen_size);

        return self.block_size;
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        let block_rect = graphics::Rect::new(0.0, 0.0, self.block_size.x, self.block_size.y);

        let block_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            block_rect,
            Color::from_rgb(
                self.selected_color.0,
                self.selected_color.1,
                self.selected_color.2,
            ),
        )
        .unwrap();

        canvas.draw(&block_mesh, Vec2::new(self.block_pos.x, self.block_pos.y));

        Ok(())
    }

    fn get_block_pos(&self, current_window_size: (f32, f32)) -> Vec2 {
        return Vec2::new(
            self.block_coord.0 as f32 * self.block_size.x,
            (self.block_coord.1 as f32 * self.block_size.y) + (current_window_size.1 * 0.1),
        );
    }

    fn get_block_size(&self, current_window_size: (f32, f32)) -> Vec2 {
        return Vec2::new(
            current_window_size.0 / BLOCK_AMOUNT_F32.0 as f32,
            (current_window_size.1 * 0.25) / BLOCK_AMOUNT_F32.1 as f32,
        );
    }
}
