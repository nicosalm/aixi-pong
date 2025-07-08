#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Up,
    Down,
    Stay,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Observation {
    pub ball_x: f32,
    pub ball_y: f32,
    pub ball_vel_x: f32,
    pub ball_vel_y: f32,
    pub paddle_y: f32,
    pub opponent_paddle_y: f32,
}

pub type Reward = f32;
