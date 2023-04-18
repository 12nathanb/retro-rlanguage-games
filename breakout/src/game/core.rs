use ggez::{
    event,
    glam::Vec2,
    graphics::{self, Color},
    Context, GameError, GameResult,
};

use crate::game::{ball::Ball, block::Block, constants::*, paddle::Paddle};

pub struct MainState {
    player: Paddle,
    ball: Ball,
    blocks: Vec<Block>,
    lives: i32,
    game_started: bool,
    current_window_size: (f32, f32),
    block_size: Vec2,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> Self {
        let mut blocks = vec![Block::default()];
        let mut temp_color = (0, 0, 0);
        let mut block_size_temp = Vec2::new(0.0, 0.0);
        let mut player = Paddle::new();
        let mut ball = Ball::new();

        for x in 0..BLOCK_AMOUNT_F32.0 {
            for y in 0..BLOCK_AMOUNT_F32.1 {
                if y == 0 || y == 1 {
                    temp_color = DARK_RED;
                } else if y == 2 || y == 3 {
                    temp_color = DARK_ORANGE;
                } else if y == 4 || y == 5 {
                    temp_color = DARK_GREEN;
                } else if y == 6 || y == 7 {
                    temp_color = DARK_YELLOW;
                }

                blocks.push(Block::new(temp_color, (x, y)))
            }
        }

        for i in 0..blocks.len() {
            block_size_temp = blocks[i].update_scale(_ctx.gfx.drawable_size());
        }

        player.update_scale(_ctx.gfx.drawable_size());
        ball.update_ball_size(_ctx.gfx.drawable_size());

        MainState {
            player: player,
            ball: ball,
            blocks: blocks,
            lives: 3,
            game_started: false,
            current_window_size: _ctx.gfx.drawable_size(),
            block_size: block_size_temp,
        }
    }

    pub fn intersects_player(&self, paddle: Vec2) -> bool {
        return self.ball.ball_pos.x - (self.ball.ball_size.x * 0.5) < paddle.x + self.block_size.x
            && self.ball.ball_pos.x + (self.ball.ball_size.x * 0.5) > paddle.x
            && self.ball.ball_pos.y - (self.ball.ball_size.y * 0.5) < paddle.y + self.block_size.y
            && self.ball.ball_pos.y + (self.ball.ball_size.y * 0.5) > paddle.y;
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.current_window_size != _ctx.gfx.drawable_size() {
            self.current_window_size = _ctx.gfx.drawable_size();

            for i in 0..self.blocks.len() {
                self.block_size = self.blocks[i].update_scale(self.current_window_size);
            }

            self.player.update_scale(self.current_window_size);
            self.ball.update_ball_size(self.current_window_size);
        }

        if self.ball.fire_ball == true && self.game_started == false {
            self.lives = 3;
        }

        self.ball
            .update(_ctx, &self.player, self.current_window_size)?;
        self.player.update(_ctx);

        for i in 0..self.blocks.len() {
            if self.intersects_player(self.blocks[i].block_pos) {
                self.blocks.remove(i);
                self.ball.reverse_velocity();
                break;
            }
        }

        if self.lives < 0 {
            self.lives = 3;
        } else if self.lives > 0 {
            if self.ball.ball_pos.y > _ctx.gfx.drawable_size().1 {
                self.ball.fire_ball = false;
                self.lives -= 1;
            }
        } else if self.lives == 0 {
            self.ball.fire_ball = false;
            self.ball.multiplyer = 0.0;
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::BLACK);
        let screen_size = (_ctx.gfx.drawable_size().0, _ctx.gfx.drawable_size().1);

        //if self.lives <= 0 {

        for block in &self.blocks {
            block.draw(_ctx, &mut canvas)?;
        }

        self.player.draw(&mut canvas, _ctx)?;

        self.ball.draw(&mut canvas, _ctx)?;

        let mut lives_text = graphics::Text::new(format!("Lives: {}", self.lives));

        lives_text.set_scale(screen_size.1 * 0.05);

        let score_pos = Vec2::new(screen_size.0 * 0.02, screen_size.1 * 0.02);

        canvas.draw(&lives_text, score_pos);
        //}

        canvas.finish(_ctx)?;

        Ok(())
    }
}
