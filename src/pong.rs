use crate::common::Observation;
use crate::config;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct GameState {
    pub ball_x: f32,
    pub ball_y: f32,
    pub ball_vel_x: f32,
    pub ball_vel_y: f32,
    pub left_paddle_y: f32,
    pub right_paddle_y: f32,
}

impl GameState {
    pub fn to_observation(&self) -> Observation {
        Observation {
            ball_x: self.ball_x,
            ball_y: self.ball_y,
            ball_vel_x: self.ball_vel_x,
            ball_vel_y: self.ball_vel_y,
            paddle_y: self.right_paddle_y,
            opponent_paddle_y: self.left_paddle_y,
        }
    }
}

struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    size: f32,
}

pub enum PaddleSide {
    Left,
    Right,
}

impl Ball {
    fn new() -> Self {
        Ball {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            vel_x: config::BALL_VEL_X,
            vel_y: config::BALL_VEL_Y,
            size: config::BALL_SIZE,
        }
    }

    fn update(&mut self, dt: f32, left_paddle_y: f32, right_paddle_y: f32) -> (bool, bool) {
        self.x += self.vel_x * dt;
        self.y += self.vel_y * dt;

        let mut left_scored = false;
        let mut right_scored = false;

        if self.y <= self.size || self.y >= screen_height() - self.size {
            self.vel_y = -self.vel_y;
        }

        if self.x <= config::PADDLE_WIDTH + self.size
            && self.y >= left_paddle_y - config::PADDLE_HEIGHT / 2.0
            && self.y <= left_paddle_y + config::PADDLE_HEIGHT / 2.0
            && self.vel_x < 0.0
        {
            self.vel_x = -self.vel_x;
        }

        if self.x >= screen_width() - config::PADDLE_WIDTH - self.size
            && self.y >= right_paddle_y - config::PADDLE_HEIGHT / 2.0
            && self.y <= right_paddle_y + config::PADDLE_HEIGHT / 2.0
            && self.vel_x > 0.0
        {
            self.vel_x = -self.vel_x;
        }

        if self.x < 0.0 {
            right_scored = true;
            self.reset();
        } else if self.x > screen_width() {
            left_scored = true;
            self.reset();
        }

        (left_scored, right_scored)
    }

    fn reset(&mut self) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
        self.vel_x = -self.vel_x;
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.size, WHITE);
    }
}

pub struct Pong {
    ball: Ball,
    left_paddle_y: f32,
    right_paddle_y: f32,
    left_score: i32,
    right_score: i32,
    paddle_speed: f32,
    score_flash_timer: f32,
    last_scorer: Option<bool>,
}

impl Pong {
    pub fn new() -> Self {
        Pong {
            ball: Ball::new(),
            left_paddle_y: screen_height() / 2.0,
            right_paddle_y: screen_height() / 2.0,
            left_score: 0,
            right_score: 0,
            paddle_speed: config::PADDLE_SPEED,
            score_flash_timer: 0.0,
            last_scorer: None,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        if self.score_flash_timer > 0.0 {
            self.score_flash_timer -= dt;
        }

        if is_key_down(KeyCode::W) && self.left_paddle_y > config::PADDLE_MARGIN {
            self.left_paddle_y -= self.paddle_speed * dt;
        }
        if is_key_down(KeyCode::S) && self.left_paddle_y < screen_height() - config::PADDLE_MARGIN {
            self.left_paddle_y += self.paddle_speed * dt;
        }

        let (left_scored, right_scored) =
            self.ball
                .update(dt, self.left_paddle_y, self.right_paddle_y);

        if left_scored {
            self.left_score += 1;
            self.score_flash_timer = 1.0;
            self.last_scorer = Some(true);
        }
        if right_scored {
            self.right_score += 1;
            self.score_flash_timer = 1.0;
            self.last_scorer = Some(false);
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        self.draw_paddles();
        self.ball.draw();
        self.draw_center_line();
        self.draw_score();

        if self.score_flash_timer > 0.0 {
            self.draw_score_effects();
        }
    }

    pub fn set_paddle(&mut self, side: PaddleSide, y: f32) {
        match side {
            PaddleSide::Left => self.left_paddle_y = y,
            PaddleSide::Right => self.right_paddle_y = y,
        }
    }

    pub fn get_state(&self) -> GameState {
        GameState {
            ball_x: self.ball.x,
            ball_y: self.ball.y,
            ball_vel_x: self.ball.vel_x,
            ball_vel_y: self.ball.vel_y,
            left_paddle_y: self.left_paddle_y,
            right_paddle_y: self.right_paddle_y,
        }
    }

    pub fn get_reward(&self) -> f32 {
        match self.last_scorer {
            Some(true) => -1.0, // left scored, bad for right agent
            Some(false) => 1.0, // right scored, good for right agent
            None => 0.0,        // no score yet
        }
    }

    fn draw_paddles(&self) {
        draw_rectangle(
            0.0,
            self.left_paddle_y - config::PADDLE_HEIGHT / 2.0,
            config::PADDLE_WIDTH,
            config::PADDLE_HEIGHT,
            WHITE,
        );

        draw_rectangle(
            screen_width() - config::PADDLE_WIDTH,
            self.right_paddle_y - config::PADDLE_HEIGHT / 2.0,
            config::PADDLE_WIDTH,
            config::PADDLE_HEIGHT,
            WHITE,
        );
    }

    fn draw_center_line(&self) {
        let center_x = screen_width() / 2.0;

        let mut y = 0.0;
        while y < screen_height() {
            draw_rectangle(center_x - 2.0, y, 4.0, config::DASH_HEIGHT, WHITE);
            y += config::DASH_HEIGHT + config::DASH_GAP;
        }
    }

    fn draw_score(&self) {
        let left_score_text = &self.left_score.to_string();
        let right_score_text = &self.right_score.to_string();

        let left_color = if self.score_flash_timer > 0.0 && self.last_scorer == Some(true) {
            GREEN
        } else {
            WHITE
        };

        let right_color = if self.score_flash_timer > 0.0 && self.last_scorer == Some(false) {
            GREEN
        } else {
            WHITE
        };

        draw_text(
            left_score_text,
            screen_width() / 4.0,
            80.0,
            config::FONT_SIZE,
            left_color,
        );

        draw_text(
            right_score_text,
            3.0 * screen_width() / 4.0,
            80.0,
            config::FONT_SIZE,
            right_color,
        );

        draw_text(
            "Left: W/S | Right: AIXI Agent",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );
    }

    fn draw_score_effects(&self) {
        if let Some(left_scored) = self.last_scorer {
            let fade = self.score_flash_timer;
            let x = if left_scored {
                screen_width() / 4.0
            } else {
                3.0 * screen_width() / 4.0
            };

            for i in 0..config::PARTICLE_COUNT {
                let angle = (i as f32 / config::PARTICLE_COUNT as f32) * 2.0 * std::f32::consts::PI;
                let distance = (1.0 - fade) * config::PARTICLE_DISTANCE;
                let particle_x = x + angle.cos() * distance;
                let particle_y = 80.0 + angle.sin() * distance;
                let size = fade * config::PARTICLE_SIZE;

                draw_circle(
                    particle_x,
                    particle_y,
                    size,
                    Color::new(1.0, 1.0, 0.0, fade),
                );
            }
        }
    }
}
