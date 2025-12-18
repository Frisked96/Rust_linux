// game_state.rs
use std::collections::HashMap;
use macroquad::prelude::*;
use crate::entity::{Pos, Player};
use crate::tile::Tile;
use crate::map_gen::{generate_dungeon, GRID_WIDTH, GRID_HEIGHT};

pub const CHAR_WIDTH: f32 = 12.0;
pub const CHAR_HEIGHT: f32 = 20.0;

pub struct GameState {
    pub player: Player,
    pub world: HashMap<Pos, Tile>,
    pub camera_x: i32,
    pub camera_y: i32,
}

impl GameState {
    pub fn new() -> Self {
        let (world, player_start) = generate_dungeon();
        Self {
            player: Player::new(player_start.x, player_start.y),
            world,
            camera_x: 0,
            camera_y: 0,
        }
    }

    pub fn update_player(&mut self) {
        let mut new_pos = self.player.pos;

        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            new_pos.y -= 1;
        }
        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            new_pos.y += 1;
        }
        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            new_pos.x -= 1;
        }
        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            new_pos.x += 1;
        }

        if self.can_move_to(new_pos) {
            self.player.pos = new_pos;
        }

        // Update camera to follow player
        self.camera_x = (self.player.pos.x - GRID_WIDTH as i32 / 2).max(0);
        self.camera_y = (self.player.pos.y - GRID_HEIGHT as i32 / 2).max(0);
    }

    pub fn can_move_to(&self, pos: Pos) -> bool {
        if pos.x < 1 || pos.x >= GRID_WIDTH as i32 - 1 {
            return false;
        }
        if pos.y < 1 || pos.y >= GRID_HEIGHT as i32 - 1 {
            return false;
        }

        match self.world.get(&pos) {
            Some(tile) => tile.char != '#',
            None => true,
        }
    }

    pub fn render(&self) {
        clear_background(BLACK);

        // Draw grid
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let world_x = x as i32 + self.camera_x;
                let world_y = y as i32 + self.camera_y;

                let pos = Pos::new(world_x, world_y);
                let tile = self.world.get(&pos).copied().unwrap_or(Tile::empty());

                // Draw background if present
                if let Some(bg) = tile.bg_color {
                    draw_rectangle(
                        x as f32 * CHAR_WIDTH,
                        y as f32 * CHAR_HEIGHT,
                        CHAR_WIDTH,
                        CHAR_HEIGHT,
                        bg,
                    );
                }

                // Draw character
                draw_text(
                    &tile.char.to_string(),
                    x as f32 * CHAR_WIDTH,
                    (y as f32 + 1.0) * CHAR_HEIGHT - 4.0,
                    CHAR_HEIGHT,
                    tile.color,
                );
            }
        }

        // Draw player
        let screen_x = (self.player.pos.x - self.camera_x) as f32;
        let screen_y = (self.player.pos.y - self.camera_y) as f32;

        if screen_x >= 0.0
            && screen_x < GRID_WIDTH as f32
            && screen_y >= 0.0
            && screen_y < GRID_HEIGHT as f32
        {
            draw_text(
                "@",
                screen_x * CHAR_WIDTH,
                (screen_y + 1.0) * CHAR_HEIGHT - 4.0,
                CHAR_HEIGHT,
                self.player.color,
            );
        }

        // Draw UI
        draw_text(
            &format!(
                "Pos: ({}, {}) | FPS: {:.0}",
                self.player.pos.x,
                self.player.pos.y,
                get_fps()
            ),
            10.0,
            screen_height() - 10.0,
            16.0,
            WHITE,
        );
    }
}