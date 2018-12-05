extern crate rand;

use std::collections::HashSet;
use tetris::piece::rand::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    x: u8,
    y: u8
}

pub trait Piece {
    fn left(&mut self) -> Box<Piece>;
    fn right(&mut self) -> Box<Piece>;
    fn down(&mut self) -> Box<Piece>;
    fn rotate(&mut self) -> Box<Piece>;
    fn spaces(&mut self) -> (Coord, Coord, Coord, Coord);
}

pub fn random(height: u8, width: u8) -> Box<Piece> {
    let r = rand::thread_rng().gen::<i8>();
    match r % 5 {
        0 => Box::new(I::new(height, width)),
        1 => Box::new(O::new(height, width)),
        _ => panic!("unreachable random piece generation")
    }
}

fn _left((s1, s2, s3, s4): (Coord, Coord, Coord, Coord)) -> (Coord, Coord, Coord, Coord) {
    let transform: Vec<Coord> = [s1, s2, s3, s4].iter()
        .map(|p| Coord{ x: p.x-1, y: p.y }).collect();
    (transform[0], transform[1], transform[2], transform[3])
}

fn _right((s1, s2, s3, s4): (Coord, Coord, Coord, Coord)) -> (Coord, Coord, Coord, Coord) {
    let transform: Vec<Coord> = [s1, s2, s3, s4].iter()
        .map(|p| Coord{ x: p.x+1, y: p.y }).collect();
    (transform[0], transform[1], transform[2], transform[3])
}

fn _down((s1, s2, s3, s4): (Coord, Coord, Coord, Coord)) -> (Coord, Coord, Coord, Coord) {
    let transform: Vec<Coord> = [s1, s2, s3, s4].iter()
        .map(|p| Coord{ x: p.x, y: p.y-1 }).collect();
    (transform[0], transform[1], transform[2], transform[3])
}

struct I {
    spaces: (Coord, Coord, Coord, Coord),
    upright: bool
}

impl I {
    fn new(height: u8, width: u8) -> impl Piece {
        let s1 = Coord{ x: width/2, y: height+2 };
        let s2 = Coord{ x: width/2, y: height+1 };
        let s3 = Coord{ x: width/2, y: height };
        let s4 = Coord{ x: width/2, y: height-0 };
        let spaces = (s1, s2, s3, s4);
        let upright = true;
        I{ spaces, upright }
    }
}

impl Piece for I {
    fn left(&mut self) -> Box<Piece> {
        let spaces = _left(self.spaces);
        Box::new(I{ spaces: spaces, upright: self.upright })
    }

    fn right(&mut self) -> Box<Piece> {
        let spaces = _right(self.spaces);
        Box::new(I{ spaces: spaces, upright: self.upright })
    }

    fn down(&mut self) -> Box<Piece> {
        let spaces = _down(self.spaces);
        Box::new(I{ spaces: spaces, upright: self.upright })
    }

    fn rotate(&mut self) -> Box<Piece> {
        let (_s1, _s2, _s3, _s4) = self.spaces;
        if self.upright {
            let s1 = Coord{ x: _s1.x-2, y: _s1.y-2 };
            let s2 = Coord{ x: _s2.x-1, y: _s2.y-1 };
            let s3 = Coord{ x: _s3.x, y: _s3.y };
            let s4 = Coord{ x: _s4.x+1, y: _s4.y+1};
            Box::new(I{ spaces: (s1, s2, s3, s4), upright: !self.upright })
        }
        else {
            // TODO: fix
            let s1 = Coord{ x: _s1.x-2, y: _s1.y-2 };
            let s2 = Coord{ x: _s2.x-1, y: _s2.y-1 };
            let s3 = Coord{ x: _s3.x, y: _s3.y };
            let s4 = Coord{ x: _s4.x+1, y: _s4.y+1};
            Box::new(I{ spaces: (s1, s2, s3, s4), upright: !self.upright })
        }
    }

    fn spaces(&mut self) -> (Coord, Coord, Coord, Coord) {
       self.spaces.clone()
    }
}

struct O {
    spaces: (Coord, Coord, Coord, Coord)
}

impl O {
    fn new(height: u8, width: u8) -> impl Piece {
        let s1 = Coord{ x: width/2, y: height+1 };
        let s2 = Coord{ x: width/2+1, y: height+1 };
        let s3 = Coord{ x: width/2, y: height };
        let s4 = Coord{ x: width/2+1, y: height };
        O{ spaces: (s1, s3, s3, s4) }
    }
}

impl Piece for O {
    fn left(&mut self) -> Box<Piece> {
        let spaces = _left(self.spaces);
        Box::new(O{ spaces: spaces })
    }

    fn right(&mut self) -> Box<Piece> {
        let spaces = _right(self.spaces);
        Box::new(O{ spaces: spaces })
    }

    fn down(&mut self) -> Box<Piece> {
        let spaces = _down(self.spaces);
        Box::new(O{ spaces: spaces })
    }

    fn rotate(&mut self) -> Box<Piece> {
        let spaces = self.spaces.clone();
        Box::new(O { spaces: spaces })
    }

    fn spaces(&mut self) -> (Coord, Coord, Coord, Coord) {
        self.spaces.clone()
    }
}
