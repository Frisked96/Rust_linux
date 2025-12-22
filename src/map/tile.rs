// tile.rs
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub char: char,
    pub color: Color,
    pub bg_color: Option<Color>,
}

impl Tile {
    pub fn new(char: char, color: Color, bg_color: Option<Color>) -> Self {
        Self {
            char,
            color,
            bg_color,
        }
    }

    pub fn empty() -> Self {
        Self {
            char: ' ',
            color: BLACK,
            bg_color: None,
        }
    }

    pub fn wall() -> Self {
        Self {
            char: '#',
            color: DARKGRAY,
            bg_color: None,
        }
    }

    pub fn floor() -> Self {
        Self {
            char: '.',
            color: LIGHTGRAY,
            bg_color: None,
        }
    }
}