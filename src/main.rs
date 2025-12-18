// main.rs
use macroquad::prelude::*;

mod tile;
mod entity;
mod map_gen;
mod game_state;

use game_state::GameState;

#[macroquad::main("Roguelike")]
async fn main() {
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