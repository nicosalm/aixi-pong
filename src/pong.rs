use macroquad::prelude::*;

struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    size: f32,
}

impl Ball {
    fn new() -> Self {
        Ball {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            vel_x: 300.0,
            vel_y: 200.0,
            size: 10.0,
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

        let paddle_width = 20.0;
        let paddle_height = 100.0;

        if self.x <= paddle_width + self.size
            && self.y >= left_paddle_y - paddle_height / 2.0
            && self.y <= left_paddle_y + paddle_height / 2.0
            && self.vel_x < 0.0 {
            self.vel_x = -self.vel_x;
        }

        if self.x >= screen_width() - paddle_width - self.size
            && self.y >= right_paddle_y - paddle_height / 2.0
            && self.y <= right_paddle_y + paddle_height / 2.0
            && self.vel_x > 0.0 {
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
            paddle_speed: 400.0,
            score_flash_timer: 0.0,
            last_scorer: None,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        if self.score_flash_timer > 0.0 {
            self.score_flash_timer -= dt;
        }

        if is_key_down(KeyCode::W) && self.left_paddle_y > 50.0 {
            self.left_paddle_y -= self.paddle_speed * dt;
        }
        if is_key_down(KeyCode::S) && self.left_paddle_y < screen_height() - 50.0 {
            self.left_paddle_y += self.paddle_speed * dt;
        }
        if is_key_down(KeyCode::Up) && self.right_paddle_y > 50.0 {
            self.right_paddle_y -= self.paddle_speed * dt;
        }
        if is_key_down(KeyCode::Down) && self.right_paddle_y < screen_height() - 50.0 {
            self.right_paddle_y += self.paddle_speed * dt;
        }

        let (left_scored, right_scored) = self.ball.update(dt, self.left_paddle_y, self.right_paddle_y);

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

    fn draw_paddles(&self) {
        let paddle_width = 20.0;
        let paddle_height = 100.0;

        draw_rectangle(
            0.0,
            self.left_paddle_y - paddle_height / 2.0,
            paddle_width,
            paddle_height,
            WHITE,
        );

        draw_rectangle(
            screen_width() - paddle_width,
            self.right_paddle_y - paddle_height / 2.0,
            paddle_width,
            paddle_height,
            WHITE,
        );
    }

    fn draw_center_line(&self) {
        let dash_height = 10.0;
        let dash_gap = 20.0;
        let center_x = screen_width() / 2.0;

        let mut y = 0.0;
        while y < screen_height() {
            draw_rectangle(center_x - 2.0, y, 4.0, dash_height, WHITE);
            y += dash_height + dash_gap;
        }
    }

    fn draw_score(&self) {
        let font_size = 60.0;
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
            font_size,
            left_color,
        );

        draw_text(
            right_score_text,
            3.0 * screen_width() / 4.0,
            80.0,
            font_size,
            right_color,
        );

        draw_text(
            "Left: W/S | Right: Arrow Keys",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );
    }

    fn draw_score_effects(&self) {
        if let Some(left_scored) = self.last_scorer {
            let fade = self.score_flash_timer;
            let x = if left_scored { screen_width() / 4.0 } else { 3.0 * screen_width() / 4.0 };

            for i in 0..20 {
                let angle = (i as f32 / 20.0) * 2.0 * std::f32::consts::PI;
                let distance = (1.0 - fade) * 100.0;
                let particle_x = x + angle.cos() * distance;
                let particle_y = 80.0 + angle.sin() * distance;
                let size = fade * 5.0;

                draw_circle(
                    particle_x,
                    particle_y,
                    size,
                    Color::new(1.0, 1.0, 0.0, fade)
                );
            }
        }
    }
}
