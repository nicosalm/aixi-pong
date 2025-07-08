// refer to: ρUCT implementation (Section 4)

use crate::common::{Action, Observation};

pub struct MCTS {
    // tree structure and search parameters
}

impl MCTS {
    pub fn new() -> Self {
        MCTS {}
    }

    pub fn search(&mut self, observation: Observation) -> Action {
        Action::Stay // placeholder until implemented
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcts_creation() {
        let _mcts = MCTS::new();
    }

    #[test]
    fn test_mcts_search_returns_action() {
        let mut mcts = MCTS::new();
        let obs = Observation {
            ball_x: 100.0,
            ball_y: 200.0,
            ball_vel_x: 1.0,
            ball_vel_y: 1.0,
            paddle_y: 300.0,
            opponent_paddle_y: 250.0,
        };

        let action = mcts.search(obs);
        // just test it returns some action
        match action {
            Action::Up | Action::Down | Action::Stay => {}
        }
    }
}
