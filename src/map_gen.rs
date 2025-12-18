// map_gen.rs
use std::collections::HashMap;
use crate::entity::Pos;
use crate::tile::Tile;
use macroquad::prelude::*;

pub const GRID_WIDTH: usize = 80;
pub const GRID_HEIGHT: usize = 24;

pub fn generate_dungeon() -> (HashMap<Pos, Tile>, Pos) {
    let mut world = HashMap::new();
    let player_start = Pos::new(40, 12);

    // Fill with empty space
    for y in 0..GRID_HEIGHT as i32 {
        for x in 0..GRID_WIDTH as i32 {
            world.insert(Pos::new(x, y), Tile::empty());
        }
    }

    // Create walls around border
    for x in 0..GRID_WIDTH as i32 {
        world.insert(Pos::new(x, 0), Tile::wall());
        world.insert(Pos::new(x, GRID_HEIGHT as i32 - 1), Tile::wall());
    }

    for y in 0..GRID_HEIGHT as i32 {
        world.insert(Pos::new(0, y), Tile::wall());
        world.insert(Pos::new(GRID_WIDTH as i32 - 1, y), Tile::wall());
    }

    // Create a simple room
    for y in 5..15 {
        for x in 10..30 {
            world.insert(Pos::new(x, y), Tile::floor());
        }
    }

    // Create a corridor
    for x in 30..60 {
        world.insert(Pos::new(x, 10), Tile::floor());
    }

    // Create another room
    for y in 8..16 {
        for x in 60..76 {
            world.insert(Pos::new(x, y), Tile::floor());
        }
    }

    // Place some monsters
    world.insert(Pos::new(20, 10), Tile::monster('g'));
    world.insert(Pos::new(70, 12), Tile::monster('o'));

    (world, player_start)
}

pub fn is_in_bounds(pos: Pos) -> bool {
    pos.x > 0 
        && pos.x < GRID_WIDTH as i32 - 1 
        && pos.y > 0 
        && pos.y < GRID_HEIGHT as i32 - 1
}
