use ggez::{
    event,
    Context,
    GameResult, 
    graphics::{self, Color, ScreenImage}, GameError, glam::Vec2
};

use crate::game::{paddle::Paddle, ball::Ball, block::{Block, self}, constants::*};

use super::constants::BLOCK_AMOUNT;

pub struct MainState {
    player: Paddle,
    ball: Ball,
    blocks: Vec<Block>,
    block_size: (f32, f32)
}

impl MainState {
    pub fn new(_ctx: &Context) -> Self {
        let mut blocks = vec![Block::default()];
        let screen_size = (_ctx.gfx.drawable_size().0, _ctx.gfx.drawable_size().1);
        

        let block_size = ((screen_size.0 * 2.0)/10.0, screen_size.1 / 10.0);

        for x in 0..BLOCK_AMOUNT_F32.0 {
            for y in 0..BLOCK_AMOUNT_F32.1 {
                blocks.push(Block::new(_ctx, Vec2::new(block_size.0, block_size.1), Vec2::new(x as f32 * block_size.0, y as f32 * block_size.1) ))
            }
        }
        print!("{}\n", _ctx.gfx.drawable_size().1);
        MainState {  
            player: Paddle::new(_ctx, Vec2::new(_ctx.gfx.drawable_size().0 * 0.5, (_ctx.gfx.drawable_size().1 * 2.0) - (RACKET_HEIGHT * 2.0))),
            ball: Ball::new(_ctx, Vec2::new(0.0,0.0)),
            blocks: blocks,
            block_size: block_size
        }
    }

    pub fn intersects_player(&self, paddle: Vec2) -> bool {
        return self.ball.ball_pos.x - BALL_SIZE_HALF < paddle.x + self.block_size.0
            && self.ball.ball_pos.x + BALL_SIZE_HALF > paddle.x
            && self.ball.ball_pos.y - BALL_SIZE_HALF < paddle.y + self.block_size.1
            && self.ball.ball_pos.y + BALL_SIZE_HALF > paddle.y;
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.ball.update(_ctx, self.player.paddle_pos)?;
        self.player.update(_ctx);

        for i in 0..self.blocks.len() {
            if self.intersects_player(self.blocks[i].block_pos) {
                self.blocks.remove(i);
                self.ball.un_reverse_velocity();
                break;
            }
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::BLACK);
        self.player.draw(&mut canvas)?;
        self.ball.draw(&mut canvas)?;
        for block in &self.blocks {
            block.draw(_ctx, &mut canvas);
        }
        canvas.finish(_ctx)?;

        Ok(())
    }
   
}

