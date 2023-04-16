use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
    Context, GameResult,
};

#[derive(Default)]
pub struct Block {
    pub block_size: Vec2,
    pub block_pos: Vec2,
    selected_color: (u8, u8, u8),
}

impl Block {
    pub fn new(block_size: Vec2, block_pos: Vec2, color: (u8, u8, u8)) -> Block {
        Block {
            block_size: block_size,
            block_pos: block_pos,
            selected_color: color,
        }
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
}
