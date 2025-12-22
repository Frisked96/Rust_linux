use macroquad::prelude::*;
use crate::map::tile::Tile;

pub const CHUNK_SIZE: i32 = 32;

#[derive(Clone)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub tiles: Vec<Tile>, // Flattened 2D array [y * CHUNK_SIZE + x]
}

impl Chunk {
    pub fn new(x: i32, y: i32) -> Self {
        let size = (CHUNK_SIZE * CHUNK_SIZE) as usize;
        let tiles = vec![Tile::wall(); size]; // Start full of walls
        Self {
            x,
            y,
            tiles,
        }
    }

    pub fn get_tile(&self, local_x: i32, local_y: i32) -> Option<&Tile> {
        if local_x < 0 || local_x >= CHUNK_SIZE || local_y < 0 || local_y >= CHUNK_SIZE {
            return None;
        }
        let idx = (local_y * CHUNK_SIZE + local_x) as usize;
        self.tiles.get(idx)
    }

    pub fn set_tile(&mut self, local_x: i32, local_y: i32, tile: Tile) {
        if local_x < 0 || local_x >= CHUNK_SIZE || local_y < 0 || local_y >= CHUNK_SIZE {
            return;
        }
        let idx = (local_y * CHUNK_SIZE + local_x) as usize;
        self.tiles[idx] = tile;
    }
}
