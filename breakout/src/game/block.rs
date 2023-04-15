use ggez::{Context, GameResult, graphics::{self, Canvas, Color}, glam::Vec2};
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct Block {
    pub block_size: Vec2,
    pub block_pos: Vec2,
    random_color: f32,
    random_color2: f32
}

impl Block {
    pub fn new(ctx: &Context, block_size: Vec2, block_pos: Vec2) -> Block {
        let mut rng = rand::thread_rng();

        Block { 
            block_size: block_size,
            block_pos: block_pos,
            random_color2: rng.gen_range(0.0, 255.0),
            random_color: rng.gen_range(0.0, 255.0)
        }

    }

    pub fn update(ball_pos: Vec2) -> GameResult {
        Ok(())
    }
    

    pub fn draw(&self, ctx: & Context, canvas: &mut Canvas) -> GameResult {
        let block_rect = graphics::Rect::new(0.0, 0.0, self.block_size.x, self.block_size.y);
        
        let c = Color::new(0.0, self.random_color, self.random_color2, 1.0);
         

        let block_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            block_rect,
            c
        ).unwrap();

        canvas.draw(&block_mesh, Vec2::new(self.block_pos.x, self.block_pos.y));
        Ok(())
    }
}