mod pong;
use pong::Pong;
use macroquad::prelude::*;

#[macroquad::main("Pong Game")]
async fn main() {
    let mut pong = Pong::new();

    loop {
        pong.update();
        pong.draw();
        next_frame().await;
    }
}
