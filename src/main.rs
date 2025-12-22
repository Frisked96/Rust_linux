// main.rs
use macroquad::prelude::*;

mod map;
mod entity;
mod game_state;

use game_state::GameState;

#[macroquad::main("Roguelike")]
async fn main() {
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
    let mut game = GameState::new();

    loop {
        game.update_player();
        game.render();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}