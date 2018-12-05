use std::collections::HashSet;
use tetris::piece::{random, Coord, Piece};

// A grid is the set of occupied coords
pub struct Grid {
    height: u8,
    width: u8,
    state: HashSet<Coord>,
    piece: Piece
}

impl Grid {
    pub fn new(height: u8, width: u8) -> Self {
        let mut state = HashSet::new();
        let piece = Box::leak(random(height, width));
        Grid { height, width, state, piece }
    }
}


