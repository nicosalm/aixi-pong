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
