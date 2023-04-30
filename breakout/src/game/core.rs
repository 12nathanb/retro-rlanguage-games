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
    score: i32,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> Self {
        let screen_size = _ctx.gfx.drawable_size();
        let mut blocks = vec![Block::default()];
        let mut block_size_temp = Vec2::new(0.0, 0.0);
        let mut player = Paddle::new();
        let mut ball = Ball::new();

        for x in 0..BLOCK_AMOUNT_F32.0 {
            for y in 0..BLOCK_AMOUNT_F32.1 {
                blocks.push(Block::new((x, y)))
            }
        }

        for i in 0..blocks.len() {
            block_size_temp = blocks[i].update_scale(screen_size);
        }

        player.update_scale(screen_size);
        ball.update_ball_size(screen_size);

        MainState {
            player: player,
            ball: ball,
            blocks: blocks,
            lives: START_LIVES,
            game_started: false,
            current_window_size: screen_size,
            block_size: block_size_temp,
            score: 0,
        }
    }

    pub fn intersects_player(&self, paddle: Vec2) -> bool {
        return self.ball.ball_pos.x - (self.ball.ball_size.x * 0.5) < paddle.x + self.block_size.x
            && self.ball.ball_pos.x + (self.ball.ball_size.x * 0.5) > paddle.x
            && self.ball.ball_pos.y - (self.ball.ball_size.y * 0.5) < paddle.y + self.block_size.y
            && self.ball.ball_pos.y + (self.ball.ball_size.y * 0.5) > paddle.y;
    }

    pub fn update_window_size(&mut self, _ctx: &mut Context) {
        if self.current_window_size != _ctx.gfx.drawable_size() {
            self.current_window_size = _ctx.gfx.drawable_size();

            for i in 0..self.blocks.len() {
                self.block_size = self.blocks[i].update_scale(self.current_window_size);
            }

            self.player.update_scale(self.current_window_size);
            self.ball.update_ball_size(self.current_window_size);
        }
    }

    pub fn check_game_started(&mut self) {
        if self.ball.fire_ball == true && self.game_started == false {
            self.lives = 3;
            self.game_started = true;
        }
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update_window_size(_ctx);
        self.check_game_started();

        self.ball
            .update(_ctx, &self.player, self.current_window_size)?;

        self.player.update(_ctx);

        for i in 0..self.blocks.len() {
            if self.intersects_player(self.blocks[i].block_pos) {
                self.blocks.remove(i);
                self.ball.reverse_velocity();
                self.score += 10;
                break;
            }
        }

        if self.lives < 0 {
            self.lives = 3;
        } else if self.lives > 0 {
            if self.ball.ball_pos.y > self.current_window_size.1 {
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

        if self.lives != 0 || !self.blocks.is_empty() {
            for block in &self.blocks {
                block.draw(_ctx, &mut canvas)?;
            }

            self.player.draw(&mut canvas, _ctx)?;
            self.ball.draw(&mut canvas, _ctx)?;

            let mut lives_text = graphics::Text::new(format!("Lives: {}", self.lives));
            let mut score_text = graphics::Text::new(format!("Score: {}", self.score));

            lives_text.set_scale(self.current_window_size.1 * 0.05);
            score_text.set_scale(self.current_window_size.1 * 0.05);

            let lives_pos = Vec2::new(
                self.current_window_size.0 * 0.02,
                self.current_window_size.1 * 0.02,
            );

            let score_pos = Vec2::new(
                self.current_window_size.0 * 0.8,
                self.current_window_size.1 * 0.02,
            );

            canvas.draw(&lives_text, lives_pos);
            canvas.draw(&score_text, score_pos);
        } else {
            let mut gameover_text = graphics::Text::new(format!("GAMEOVER\nScore:{}", self.score));
            gameover_text.set_scale(self.current_window_size.1 * 0.2);

            let gameover_pos = Vec2::new(
                self.current_window_size.0 * 0.17,
                self.current_window_size.1 * 0.35,
            );

            canvas.draw(&gameover_text, gameover_pos);
        }
        canvas.finish(_ctx)?;

        Ok(())
    }
}
