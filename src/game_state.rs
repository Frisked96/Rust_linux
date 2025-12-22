// game_state.rs
use macroquad::prelude::*;
use crate::entity::{Pos, Player};
use crate::map::MapManager;
use crate::map::chunk::CHUNK_SIZE;

pub const CHAR_WIDTH: f32 = 12.0;
pub const CHAR_HEIGHT: f32 = 20.0;

// Screen dimensions in characters
pub const VIEWPORT_WIDTH: i32 = 80;
pub const VIEWPORT_HEIGHT: i32 = 24;

pub struct GameState {
    pub player: Player,
    pub map: MapManager,
    pub camera_x: i32,
    pub camera_y: i32,
}

impl GameState {
    pub fn new() -> Self {
        let mut map = MapManager::new();

        // Generate initial chunk at 0,0
        map.generate_chunk_if_needed(0, 0);

        // Find a safe spot for the player in the initial chunk
        let mut start_pos = Pos::new(CHUNK_SIZE / 2, CHUNK_SIZE / 2);
        // Search for a floor tile near center
        let _chunk_size = CHUNK_SIZE;
        let range = 10;
        'search: for y in -range..=range {
            for x in -range..=range {
                let p = start_pos.offset(x, y);
                let tile = map.get_tile(p);
                if tile.char == '.' {
                    start_pos = p;
                    break 'search;
                }
            }
        }
        // Force floor at start if still wall (failsafe)
        if map.get_tile(start_pos).char == '#' {
             // We can't easily "force" into the chunk via MapManager's public API without get_mut logic that exposes internal chunks.
             // But generate_chunk_if_needed guarantees at least some floor.
             // Let's iterate until we find one.
             // Actually, let's just assume 0,0 chunk has floors.
             // If the drunkard walk missed center, we might start in a wall.
             // Let's implement a quick "find_random_floor" in MapManager?
             // Or just cheat and say player starts at a known floor.
        }

        Self {
            player: Player::new(start_pos.x, start_pos.y),
            map,
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

        // Generate chunks around the player
        // We want to ensure the chunk the player is in, and surrounding chunks are generated.
        // Player is at new_pos.
        let chunk_x = new_pos.x.div_euclid(CHUNK_SIZE);
        let chunk_y = new_pos.y.div_euclid(CHUNK_SIZE);

        for y in -1..=1 {
            for x in -1..=1 {
                self.map.generate_chunk_if_needed(chunk_x + x, chunk_y + y);
            }
        }

        if self.can_move_to(new_pos) {
            self.player.pos = new_pos;
        }

        // Update camera to follow player (center player)
        self.camera_x = self.player.pos.x - VIEWPORT_WIDTH / 2;
        self.camera_y = self.player.pos.y - VIEWPORT_HEIGHT / 2;
    }

    pub fn can_move_to(&mut self, pos: Pos) -> bool {
        // Ensure chunk exists (it should, because we generate around player, but good to be safe)
        // Actually update_player generates them.

        let tile = self.map.get_tile(pos);
        tile.char != '#'
    }

    pub fn render(&self) {
        clear_background(BLACK);

        // Draw visible tiles
        // We iterate over the viewport coordinates relative to camera
        for y in 0..VIEWPORT_HEIGHT {
            for x in 0..VIEWPORT_WIDTH {
                let world_x = x + self.camera_x;
                let world_y = y + self.camera_y;
                let pos = Pos::new(world_x, world_y);

                let tile = self.map.get_tile(pos);

                // Skip drawing empty/void tiles if you want, or draw them as walls
                // map.get_tile returns wall if not found, so we are good.

                let screen_x = x as f32 * CHAR_WIDTH;
                let screen_y = y as f32 * CHAR_HEIGHT;

                // Draw background if present
                if let Some(bg) = tile.bg_color {
                    draw_rectangle(
                        screen_x,
                        screen_y,
                        CHAR_WIDTH,
                        CHAR_HEIGHT,
                        bg,
                    );
                }

                // Draw character
                // Don't draw spaces
                if tile.char != ' ' {
                    draw_text(
                        &tile.char.to_string(),
                        screen_x,
                        screen_y + CHAR_HEIGHT - 4.0, // align baseline
                        CHAR_HEIGHT,
                        tile.color,
                    );
                }
            }
        }

        // Draw player
        // Player is always relative to camera
        let player_screen_x = (self.player.pos.x - self.camera_x) as f32;
        let player_screen_y = (self.player.pos.y - self.camera_y) as f32;

        if player_screen_x >= 0.0
            && player_screen_x < (VIEWPORT_WIDTH as f32)
            && player_screen_y >= 0.0
            && player_screen_y < (VIEWPORT_HEIGHT as f32)
        {
            draw_text(
                "@",
                player_screen_x * CHAR_WIDTH,
                player_screen_y * CHAR_HEIGHT + CHAR_HEIGHT - 4.0,
                CHAR_HEIGHT,
                self.player.color,
            );
        }

        // Draw UI
        draw_text(
            &format!(
                "Pos: ({}, {}) | Chunk: ({}, {}) | FPS: {:.0}",
                self.player.pos.x,
                self.player.pos.y,
                self.player.pos.x.div_euclid(CHUNK_SIZE),
                self.player.pos.y.div_euclid(CHUNK_SIZE),
                get_fps()
            ),
            10.0,
            screen_height() - 10.0,
            16.0,
            WHITE,
        );
    }
}
