extern crate rand;

use std::collections::HashSet;
use grid::rand::prelude::*;

#[derive(PartialEq, Eq, Hash)]
pub struct Coord {
    x: u8,
    y: u8
}

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
        let mut piece = Piece::random(height, width);
        Grid { height, width, state, piece }
    }
}

pub struct Piece {
    spaces: HashSet<Coord>
}

impl Piece {
    pub fn random(height: u8, width: u8) -> Self {
        let r = rand::thread_rng().gen::<i8>();
        let mut spaces = HashSet::new();
        match r % 5 {
            // ####
            0 => {
                spaces.insert(Coord{ x: width/2, y: height+1 });
                spaces.insert(Coord{ x: width/2-1, y: height+1 });
                spaces.insert(Coord{ x: width/2+1, y: height+1 });
                spaces.insert(Coord{ x: width/2+2, y: height+1 });
            }
            // ##
            //  ##
            1 => {
                spaces.insert(Coord{ x: width/2, y: height+1 });
                spaces.insert(Coord{ x: width/2-1, y: height+1 });
                spaces.insert(Coord{ x: width/2, y: height });
                spaces.insert(Coord{ x: width/2+1, y: height });
            }
            //  ##
            // ##
            2 => {
                spaces.insert(Coord{ x: width/2, y: height+1 });
                spaces.insert(Coord{ x: width/2+1, y: height+1 });
                spaces.insert(Coord{ x: width/2, y: height });
                spaces.insert(Coord{ x: width/2-1, y: height });
            }
            //  #
            // ###
            3 => {
                spaces.insert(Coord{ x: width/2, y: height+1 });
                spaces.insert(Coord{ x: width/2, y: height });
                spaces.insert(Coord{ x: width/2-1, y: height });
                spaces.insert(Coord{ x: width/2+1, y: height });
            }
            // ##
            // ##
            4 => {
                spaces.insert(Coord{ x: width/2, y: height+1 });
                spaces.insert(Coord{ x: width/2+1, y: height+1 });
                spaces.insert(Coord{ x: width/2, y: height });
                spaces.insert(Coord{ x: width/2+1, y: height });
            }
            _ => panic!("unreachable random piece generation")
        }
        Piece { spaces }
    }
}

fn tick(grid: &mut Grid) -> (i32, bool) {
    return (0, true);
}
