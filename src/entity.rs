// entity.rs
use macroquad::prelude::Color;
use std::collections::HashMap;
use crate::tile::Tile;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn offset(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

pub struct Player {
    pub pos: Pos,
    pub char: char,
    pub color: Color,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: Pos::new(x, y),
            char: '@',
            color: macroquad::prelude::GREEN,
        }
    }

    pub fn can_move_to(&self, pos: Pos, world: &HashMap<Pos, Tile>) -> bool {
        if let Some(tile) = world.get(&pos) {
            tile.char != '#'
        } else {
            true
        }
    }
}