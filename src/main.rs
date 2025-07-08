mod agent;
mod common;
mod config;
mod pong;

use agent::Agent;
use common::Action;
use pong::{PaddleSide, Pong};

use macroquad::prelude::*;

#[macroquad::main("AIXI Pong")]
async fn main() {
    let mut pong = Pong::new();
    let mut agent = Agent::new();

    loop {
        let state = pong.get_state();
        let observation = state.to_observation();
        let reward = pong.get_reward();

        agent.update(reward);
        let action = agent.step(observation);

        let dt = get_frame_time();
        let mut new_y = state.right_paddle_y;

        match action {
            Action::Up => new_y -= config::PADDLE_SPEED * dt,
            Action::Down => new_y += config::PADDLE_SPEED * dt,
            Action::Stay => {}
        }

        pong.set_paddle(PaddleSide::Right, new_y);

        pong.update();
        pong.draw();
        next_frame().await;
    }
}
