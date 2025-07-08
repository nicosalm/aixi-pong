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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_equality() {
        assert_eq!(Action::Up, Action::Up);
        assert_ne!(Action::Up, Action::Down);
        assert_ne!(Action::Down, Action::Stay);
    }

    #[test]
    fn test_observation_creation() {
        let obs = Observation {
            ball_x: 100.0,
            ball_y: 200.0,
            ball_vel_x: 5.0,
            ball_vel_y: -3.0,
            paddle_y: 300.0,
            opponent_paddle_y: 250.0,
        };

        assert_eq!(obs.ball_x, 100.0);
        assert_eq!(obs.ball_y, 200.0);
        assert_eq!(obs.ball_vel_x, 5.0);
        assert_eq!(obs.paddle_y, 300.0);
    }
}
