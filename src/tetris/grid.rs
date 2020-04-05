use std::collections::{HashMap, HashSet};
use tetris::piece::{down, into_set, left, random, right, rotate_left, rotate_right, Coord, Piece};

// A grid is the set of occupied coords
pub struct Grid {
    height: i8,
    width: i8,
    pub state: HashSet<Coord>,
    pub piece: Piece,
    pub next_piece: Piece,
}

impl Grid {
    pub fn new(height: i8, width: i8) -> Self {
        let state = HashSet::new();
        let piece = random(width);
        let next_piece = random(width);
        Grid {
            height,
            width,
            state,
            piece,
            next_piece,
        }
    }

    pub fn rotate_left(&mut self) {
        let piece = self.piece.to_owned();
        let aspirational_piece = rotate_left(piece);
        let aspirational_set = into_set(&piece);
        if self.state.intersection(&into_set(&piece)).count() == 0 {
            self.piece = aspirational_piece;
            if aspirational_set.iter().any(|coord| coord.x < 0) {
                self.right();
            }
            if aspirational_set.iter().any(|coord| coord.x > self.width) {
                self.left();
            }
        }
    }

    pub fn rotate_right(&mut self) {
        let piece = self.piece.to_owned();
        let aspirational_piece = rotate_right(piece);
        let aspirational_set = into_set(&piece);
        if self.state.intersection(&into_set(&piece)).count() == 0 {
            self.piece = aspirational_piece;
            if aspirational_set.iter().any(|coord| coord.x < 0) {
                self.right();
            }
            if aspirational_set.iter().any(|coord| coord.x > self.width) {
                self.left();
            }
        }
    }

    pub fn left(&mut self) {
        let piece = self.piece.to_owned();
        let aspirational_piece = left(piece);
        let aspirational_coords = into_set(&aspirational_piece);
        if aspirational_coords.iter().all(|c| c.x >= 0)
            && self.state.intersection(&aspirational_coords).count() == 0
        {
            self.piece = aspirational_piece;
        }
    }

    pub fn right(&mut self) {
        let piece = self.piece.to_owned();
        let aspirational_piece = right(piece);
        let aspirational_coords = into_set(&aspirational_piece);
        if aspirational_coords.iter().all(|c| c.x < self.width)
            && self.state.intersection(&aspirational_coords).count() == 0
        {
            self.piece = aspirational_piece;
        }
    }

    pub fn down(&mut self) -> bool {
        let piece = self.piece.to_owned();
        let aspirational_piece = down(piece);
        let aspirational_coords = into_set(&aspirational_piece);
        let ret = aspirational_coords.iter().all(|c| c.y < self.height)
            && self.state.intersection(&aspirational_coords).count() == 0;
        if ret {
            self.piece = aspirational_piece;
        }
        ret
    }

    pub fn drop(&mut self) {
        let mut current_coords = into_set(&self.piece);
        self.down();
        let mut next_coords = into_set(&self.piece);
        while current_coords != next_coords {
            current_coords = next_coords;
            self.down();
            next_coords = into_set(&self.piece);
        }
    }

    pub fn tick(&mut self) -> usize {
        if !self.down() {
            let next_coords = into_set(&self.piece);
            let next_state: HashSet<Coord> =
                self.state.union(&next_coords).map(|coord| *coord).collect();
            let line_counts = next_state.iter().fold(HashMap::new(), |mut acc, i| {
                let v = acc.get(&i.y).unwrap_or(&0).to_owned();
                acc.insert(i.y, v + 1);
                acc
            });
            let dead_lines: Vec<&i8> = line_counts
                .iter()
                .filter(|(_, count)| **count == self.width)
                .map(|(line, _)| line)
                .collect();
            self.state = next_state;
            self.piece = self.next_piece;
            self.next_piece = random(self.width);
            dead_lines.len()
        } else {
            0
        }
    }
}
