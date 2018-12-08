use std::collections::HashSet;
use tetris::piece::{down, left, into_set, random, right, rotate, Coord, Piece};

// A grid is the set of occupied coords
pub struct Grid {
    height: u8,
    width: u8,
    state: HashSet<Coord>,
    piece: Piece,
    next_piece: Piece
}

impl Grid {
    pub fn new(height: u8, width: u8) -> Self {
        let mut state = HashSet::new();
        let piece = random(height, width);
        let next_piece = random(height, width);
        Grid { height, width, state, piece, next_piece }
    }

    pub fn rotate(&mut self) {
        // TODO: check bounds
        let piece = self.piece.to_owned();
        let aspirational_piece = rotate(piece);
        if self.state.intersection(&into_set(piece)).count() == 0 {
            self.piece = aspirational_piece;
        }
    }

    pub fn left(&mut self) {
        let piece = self.piece.to_owned();
        let aspirational_piece = left(piece);
        let aspirational_coords = into_set(aspirational_piece);
        if aspirational_coords.iter().all(|c| c.x >= 0) &&
            self.state.intersection(&aspirational_coords).count() == 0 {
            self.piece = aspirational_piece;
        }
    }

    pub fn right(&mut self) {
        let piece = self.piece.to_owned();
        let aspirational_piece = right(piece);
        let aspirational_coords = into_set(aspirational_piece);
        if aspirational_coords.iter().all(|c| c.x < self.width) &&
            self.state.intersection(&aspirational_coords).count() == 0 {
            self.piece = aspirational_piece;
        }
    }

    pub fn down(&mut self) {
        let piece = self.piece.to_owned();
        let aspirational_piece = down(piece);
        let aspirational_coords = into_set(aspirational_piece);
        if aspirational_coords.iter().all(|c| c.y >= 0) &&
            self.state.intersection(&aspirational_coords).count() == 0 {
            self.piece = aspirational_piece;
        }
    }

    pub fn drop(&mut self) {
        let mut current_coords = into_set(self.piece.to_owned());
        self.down();
        let mut next_coords = into_set(self.piece.to_owned());
        while current_coords != next_coords {
            current_coords = next_coords;
            self.down();
            next_coords = into_set(self.piece.to_owned());
        }
    }

    pub fn tick(&mut self) -> u32 {
        let current_coords = into_set(self.piece.to_owned());
        self.down();
        let next_coords = into_set(self.piece.to_owned());
        if current_coords == next_coords {
            // check for row clears
            // move piece coords into grid state
            // move next piece to piece and make new next
            0
        } else {
            0
        }
    }
}
