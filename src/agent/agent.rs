// refer to: main MC-AIXI agent (Section 6)

use crate::common::{Action, Observation, Reward};

pub struct Agent {
    // mcts and ctw components will go here
}

impl Agent {
    pub fn new() -> Self {
        Agent {
            // TODO: initialize components
        }
    }

    pub fn step(&mut self, observation: Observation) -> Action {
        // simple heuristic for now, replace with mcts later
        if observation.paddle_y < observation.ball_y {
            Action::Down
        } else if observation.paddle_y > observation.ball_y {
            Action::Up
        } else {
            Action::Stay
        }
    }

    pub fn update(&mut self, reward: Reward) {
        // feed reward back to learning components
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let _agent = Agent::new();
        // just test it doesn't panic
    }

    #[test]
    fn test_agent_simple_logic() {
        let mut agent = Agent::new();

        // ball above paddle -> should move up
        let obs_up = Observation {
            ball_x: 400.0,
            ball_y: 100.0,
            ball_vel_x: 1.0,
            ball_vel_y: 1.0,
            paddle_y: 200.0,
            opponent_paddle_y: 150.0,
        };

        let action = agent.step(obs_up);
        assert_eq!(action, Action::Up);

        // ball below paddle -> should move down
        let obs_down = Observation {
            ball_x: 400.0,
            ball_y: 300.0,
            ball_vel_x: 1.0,
            ball_vel_y: 1.0,
            paddle_y: 200.0,
            opponent_paddle_y: 150.0,
        };

        let action = agent.step(obs_down);
        assert_eq!(action, Action::Down);

        // ball at same level -> should stay
        let obs_stay = Observation {
            ball_x: 400.0,
            ball_y: 200.0,
            ball_vel_x: 1.0,
            ball_vel_y: 1.0,
            paddle_y: 200.0,
            opponent_paddle_y: 150.0,
        };

        let action = agent.step(obs_stay);
        assert_eq!(action, Action::Stay);
    }

    #[test]
    fn test_reward_update() {
        let mut agent = Agent::new();

        // test that reward update doesn't panic
        agent.update(1.0);
        agent.update(-1.0);
        agent.update(0.0);
    }
}
