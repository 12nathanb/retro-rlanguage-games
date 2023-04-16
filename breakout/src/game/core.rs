use ggez::{
    event,
    Context,
    GameResult, 
    graphics::{self, Color}, GameError, glam::Vec2
};

use crate::game::{paddle::Paddle, ball::Ball, block::{Block}, constants::*};

pub struct MainState {
    player: Paddle,
    ball: Ball,
    blocks: Vec<Block>,
    block_size: (f32, f32),
    lives: i32
}

impl MainState {
    pub fn new(_ctx: &Context) -> Self {
        let mut blocks = vec![Block::default()];
        let screen_size = (_ctx.gfx.drawable_size().0, _ctx.gfx.drawable_size().1);
        

        let block_size = ((screen_size.0 * 2.0)/14.0, (screen_size.1 * 0.5)/ 8.0);

        let mut temp_color = (0, 0, 0);

        for x in 0..BLOCK_AMOUNT_F32.0 {
            for y in 0..BLOCK_AMOUNT_F32.1 {
                if y == 0 || y == 1 {
                    temp_color = DARK_RED;
                }

                if y == 2 || y == 3 {
                    temp_color = DARK_ORANGE;
                }

                if y == 4 || y == 5 {
                    temp_color = DARK_GREEN;
                }

                if y == 6 || y == 7 {
                    temp_color = DARK_YELLOW;
                }

                blocks.push(Block::new(Vec2::new(
                    block_size.0 - 20.0, block_size.1 - 20.0), 
                    Vec2::new(x as f32 * block_size.0, (y as f32 * block_size.1) + 200.0), 
                    temp_color))
                
            }
        }
        print!("{}\n", _ctx.gfx.drawable_size().1);
        MainState {  
            player: Paddle::new(_ctx, Vec2::new(_ctx.gfx.drawable_size().0 * 0.5, (_ctx.gfx.drawable_size().1 * 2.0) - (RACKET_HEIGHT * 2.0))),
            ball: Ball::new(_ctx, Vec2::new(0.0,0.0)),
            blocks: blocks,
            block_size: block_size,
            lives: 3
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
        self.ball.update(_ctx, self.player.paddle_pos, self.player.paddle_vel)?;
        self.player.update(_ctx);

        print!("{} {}\n", self.player.paddle_rect.x, self.player.paddle_rect.y);
        if self.ball.ball_pos.y >= _ctx.gfx.drawable_size().1 {
            print!("Life lost\n");
            self.lives -= 1;
        }

        for i in 0..self.blocks.len() {
            if self.intersects_player(self.blocks[i].block_pos) {
                self.blocks.remove(i);
                self.ball.reverse_velocity();
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
            block.draw(_ctx, &mut canvas)?;
        }
    
        
        canvas.finish(_ctx)?;

        Ok(())
    }
   
}

