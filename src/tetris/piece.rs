extern crate rand;

use std::collections::HashSet;
use tetris::piece::rand::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    pub x: i8,
    pub y: i8
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union Piece {
    i: I,
    o: O,
    t: T,
    s: S,
    z: Z,
    j: J,
    l: L
}

#[derive(Clone, Copy)]
struct State {
    coords: (Coord, Coord, Coord, Coord),
    position: u8
}

#[derive(Clone, Copy)]
struct I { state: State }
#[derive(Clone, Copy)]
struct O { state: State }
#[derive(Clone, Copy)]
struct T { state: State }
#[derive(Clone, Copy)]
struct S { state: State }
#[derive(Clone, Copy)]
struct Z { state: State }
#[derive(Clone, Copy)]
struct J { state: State }
#[derive(Clone, Copy)]
struct L { state: State }

impl I {
    fn new(width: i8) -> Self {
        let coords = (Coord{ x: width/2, y: -2 },
                      Coord{ x: width/2, y: -1 },
                      Coord{ x: width/2, y: 0 },
                      Coord{ x: width/2, y: 1 });
        let state = State{ coords: coords, position: 0 };
        I{ state }
    }
}

impl O {
    fn new(width: i8) -> Self {
        let coords = (Coord{ x: width/2, y: -1 },
                      Coord{ x: width/2+1, y: -1 },
                      Coord{ x: width/2, y: 0 },
                      Coord{ x: width/2+1, y: 0 });
        let state = State{ coords: coords, position: 0 };
        O{ state }
    }
}

impl T {
    fn new(width: i8) -> Self {
        let coords = (Coord{ x: width/2, y: -1 },
                      Coord{ x: width/2-1, y: 0 },
                      Coord{ x: width/2, y: 0 },
                      Coord{ x: width/2+1, y: 0 });
        let state = State{ coords: coords, position: 0 };
        T{ state }
    }
}

impl S {
    fn new(width: i8) -> Self {
        let coords = (Coord{ x: width/2, y: -1 },
                      Coord{ x: width/2+1, y: -1 },
                      Coord{ x: width/2-1, y: 0 },
                      Coord{ x: width/2, y: 0 });
        let state = State{ coords: coords, position: 0 };
        S{ state }
    }
}

impl Z {
    fn new(width: i8) -> Self {
        let coords = (Coord{ x: width/2-1, y: -1 },
                      Coord{ x: width/2, y: -1 },
                      Coord{ x: width/2, y: 0 },
                      Coord{ x: width/2+1, y: 0 });
        let state = State{ coords: coords, position: 0 };
        Z{ state }
    }
}

impl J {
    fn new(width: i8) -> Self {
        let coords = (Coord{ x: width/2-1, y: -1 },
                      Coord{ x: width/2-1, y: 0 },
                      Coord{ x: width/2, y: 0 },
                      Coord{ x: width/2+1, y: 0 });
        let state = State{ coords: coords, position: 0 };
        J{ state }
    }
}

impl L {
    fn new(width: i8) -> Self {
        let coords = (Coord{ x: width/2+1, y: -1 },
                      Coord{ x: width/2+1, y: 0 },
                      Coord{ x: width/2, y: 0 },
                      Coord{ x: width/2-1, y: 0 });
        let state = State{ coords: coords, position: 0 };
        L{ state }
    }
}

pub fn random(width: i8) -> Piece {
    let r = rand::thread_rng().next_u32();
    match r % 7 {
        0 => Piece{ i: I::new(width) },
        1 => Piece{ o: O::new(width) },
        2 => Piece{ t: T::new(width) },
        3 => Piece{ s: S::new(width) },
        4 => Piece{ z: Z::new(width) },
        5 => Piece{ j: J::new(width) },
        6 => Piece{ l: L::new(width) },
        _ => panic!("unreachable random piece generation")
    }
}

fn _left(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    let coords = (Coord{ x: s1.x-1, y: s1.y },
                  Coord{ x: s2.x-1, y: s2.y },
                  Coord{ x: s3.x-1, y: s3.y },
                  Coord{ x: s4.x-1, y: s4.y });
    State{ coords: coords, position: state.position }
}

pub fn left(piece: Piece) -> Piece {
    unsafe {
        match piece {
            Piece { i } => { Piece{ i: I{ state: _left(i.state) } } }
            Piece { o } => { Piece{ o: O{ state: _left(o.state) } } }
            Piece { t } => { Piece{ t: T{ state: _left(t.state) } } }
            Piece { s } => { Piece{ s: S{ state: _left(s.state) } } }
            Piece { z } => { Piece{ z: Z{ state: _left(z.state) } } }
            Piece { j } => { Piece{ j: J{ state: _left(j.state) } } }
            Piece { l } => { Piece{ l: L{ state: _left(l.state) } } }
        }
    }
}

fn _right(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    let coords = (Coord{ x: s1.x+1, y: s1.y },
                  Coord{ x: s2.x+1, y: s2.y },
                  Coord{ x: s3.x+1, y: s3.y },
                  Coord{ x: s4.x+1, y: s4.y });
    State{ coords: coords, position: state.position }
}

pub fn right(piece: Piece) -> Piece {
    unsafe {
        match piece {
            Piece { i } => { Piece{ i: I{ state: _right(i.state) } } }
            Piece { o } => { Piece{ o: O{ state: _right(o.state) } } }
            Piece { t } => { Piece{ t: T{ state: _right(t.state) } } }
            Piece { s } => { Piece{ s: S{ state: _right(s.state) } } }
            Piece { z } => { Piece{ z: Z{ state: _right(z.state) } } }
            Piece { j } => { Piece{ j: J{ state: _right(j.state) } } }
            Piece { l } => { Piece{ l: L{ state: _right(l.state) } } }
        }
    }
}

fn _down(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    let coords = (Coord{ x: s1.x, y: s1.y+1 },
                  Coord{ x: s2.x, y: s2.y+1 },
                  Coord{ x: s3.x, y: s3.y+1 },
                  Coord{ x: s4.x, y: s4.y+1 });
    State{ coords: coords, position: state.position }
}

pub fn down(piece: Piece) -> Piece {
    unsafe {
        match piece {
            Piece { i } => { Piece{ i: I{ state: _down(i.state) } } }
            Piece { o } => { Piece{ o: O{ state: _down(o.state) } } }
            Piece { t } => { Piece{ t: T{ state: _down(t.state) } } }
            Piece { s } => { Piece{ s: S{ state: _down(s.state) } } }
            Piece { z } => { Piece{ z: Z{ state: _down(z.state) } } }
            Piece { j } => { Piece{ j: J{ state: _down(j.state) } } }
            Piece { l } => { Piece{ l: L{ state: _down(l.state) } } }
        }
    }
}

fn rotate_i(i: I) -> Piece {
    let (s1, s2, s3, s4) = i.state.coords;
    if i.state.position == 0 {
        let coords = (Coord{ x: s1.x-2, y: s1.y+2 },
                      Coord{ x: s2.x-1, y: s2.y+1 },
                      s3,
                      Coord{ x: s4.x+1, y: s4.y-1 });
        let state = State{ coords: coords, position: 0 };
        Piece{ i: I{ state } }
    } else {
        let coords = (Coord{ x: s1.x+2, y: s1.y-2 },
                      Coord{ x: s2.x+1, y: s2.y-1 },
                      s3,
                      Coord{ x: s4.x-1, y: s4.y+1 });
        let state = State{ coords: coords, position: 1 };
        Piece{ i: I{ state } }
    }
}

fn rotate_t(t: T) -> Piece {
    let (s1, s2, s3, s4) = t.state.coords;
    match t.state.position {
        0 => {
            let coords = (Coord{ x: s1.x-1, y: s1.y+1 },
                          Coord{ x: s2.x+1, y: s2.y+1 },
                          s3,
                          Coord{ x: s4.x-1, y: s4.y-1 });
            let state = State{ coords: coords, position: 1 };
            Piece{ t: T{ state } }
        },
        1 => {
            let coords = (Coord{ x: s1.x+1, y: s1.y+1 },
                          Coord{ x: s2.x+1, y: s2.y-1 },
                          s3,
                          Coord{ x: s4.x-1, y: s4.y+1 });
            let state = State{ coords: coords, position: 2 };
            Piece{ t: T{ state } }
        },
        2 => {
            let coords = (Coord{ x: s1.x+1, y: s1.y-1 },
                          Coord{ x: s2.x-1, y: s2.y-1 },
                          s3,
                          Coord{ x: s4.x+1, y: s4.y+1});
            let state = State{ coords: coords, position: 3 };
            Piece{ t: T{ state } }
        },
        3 => {
            let coords = (Coord{ x: s1.x-1, y: s1.y-1 },
                          Coord{ x: s2.x-1, y: s2.y+1 },
                          s3,
                          Coord{ x: s4.x+1, y: s4.y-1 });
            let state = State{ coords: coords, position: 0 };
            Piece{ t: T{ state } }
        }
        _ => panic!("Invalid T position")
    }
}

fn rotate_s(s: S) -> Piece {
    let (s1, s2, s3, s4) = s.state.coords;
    if s.state.position == 0 {
        let coords = (s1,
                      Coord{ x: s2.x-1, y: s2.y-1 },
                      Coord{ x: s3.x+1, y: s3.y },
                      Coord{ x: s4.x+1, y: s4.y-1 });
        let state = State{ coords: coords, position: 1 };
        Piece{ s: S{ state } }
    } else {
        let coords = (s1,
                      Coord{ x: s2.x+1, y: s2.y+1 },
                      Coord{ x: s3.x-1, y: s3.y },
                      Coord{ x: s4.x-1, y: s4.y+1 });
        let state = State{ coords: coords, position: 0 };
        Piece{ s: S{ state } }
    }
}

fn rotate_z(z: Z) -> Piece {
    let (s1, s2, s3, s4) = z.state.coords;
    if z.state.position == 0 {
        let coords = (Coord{ x: s1.x+1, y: s1.y-1 },
                      s2,
                      Coord{ x: s3.x+1, y: s3.y+1 },
                      Coord{ x: s4.x-2, y: s4.y });
        let state = State{ coords: coords, position: 1 };
        Piece{ z: Z{ state } }
    } else {
        let coords = (Coord{ x: s1.x-1, y: s1.y+1 },
                      s2,
                      Coord{ x: s3.x-1, y: s3.y-1 },
                      Coord{ x: s4.x+1, y: s4.y });
        let state = State{ coords: coords, position: 0 };
        Piece{ z: Z{ state } }
    }
}

fn rotate_j(j: J) -> Piece {
    let (s1, s2, s3, s4) = j.state.coords;
    match j.state.position {
        0 => {
            let coords = (Coord{ x: s1.x-1, y: s1.y+1 },
                          s2,
                          Coord{ x: s3.x-1, y: s3.y-1 },
                          Coord{ x: s4.x-2, y: s4.y-2 });
            let state = State{ coords: coords, position: 1 };
            Piece{ j: J{ state } }
        },
        1 => {
            let coords = (Coord{ x: s1.x+1, y: s1.y+1 },
                          s2,
                          Coord{ x: s3.x-1, y: s3.y+1 },
                          Coord{ x: s4.x-2, y: s4.y+2 });
            let state = State{ coords: coords, position: 2 };
            Piece{ j: J{ state } }
        },
        2 => {
            let coords = (Coord{ x: s1.x+1, y: s1.y-1 },
                          s2,
                          Coord{ x: s3.x+1, y: s3.y+1 },
                          Coord{ x: s4.x+2, y: s4.y+2 });
            let state = State{ coords: coords, position: 3 };
            Piece{ j: J{ state } }
        },
        3 => {
            let coords = (Coord{ x: s1.x-1, y: s1.y-1 },
                          s2,
                          Coord{ x: s3.x+1, y: s3.y-1 },
                          Coord{ x: s4.x+2, y: s4.y-2 });
            let state = State{ coords: coords, position: 0 };
            Piece{ j: J{ state } }
        }
        _ => panic!("Invalid J position")
    }
}

fn rotate_l(l: L) -> Piece {
    let (s1, s2, s3, s4) = l.state.coords;
    match l.state.position {
        0 => {
            let coords = (Coord{ x: s1.x-1, y: s1.y+1 },
                          s2,
                          Coord{ x: s3.x+1, y: s3.y+1 },
                          Coord{ x: s4.x+2, y: s4.y+2 });
            let state = State{ coords: coords, position: 1 };
            Piece{ l: L{ state } }
        },
        1 => {
            let coords = (Coord{ x: s1.x+1, y: s1.y+1 },
                          s2,
                          Coord{ x: s3.x+1, y: s3.y-1 },
                          Coord{ x: s4.x+2, y: s4.y-2 });
            let state = State{ coords: coords, position: 2 };
            Piece{ l: L{ state } }
        },
        2 => {
            let coords = (Coord{ x: s1.x+1, y: s1.y-1 },
                          s2,
                          Coord{ x: s3.x-1, y: s3.y-1 },
                          Coord{ x: s4.x-2, y: s4.y-2 });
            let state = State{ coords: coords, position: 3 };
            Piece{ l: L{ state } }
        },
        3 => {
            let coords = (Coord{ x: s1.x-1, y: s1.y-1 },
                          s2,
                          Coord{ x: s3.x-1, y: s3.y+1 },
                          Coord{ x: s4.x-2, y: s4.y+2 });
            let state = State{ coords: coords, position: 0 };
            Piece{ l: L{ state } }
        }
        _ => panic!("Invalid L position")
    }
}

pub fn rotate(piece: Piece) -> Piece {
    unsafe {
        match piece {
            Piece { i } => { rotate_i(i) }
            Piece { o } => { Piece{ o: o } }
            Piece { t } => { rotate_t(t) }
            Piece { s } => { rotate_s(s) }
            Piece { z } => { rotate_z(z) }
            Piece { j } => { rotate_j(j) }
            Piece { l } => { rotate_l(l) }
        }
    }
}

fn _into_set(state: State) -> HashSet<Coord> {
    let (s1, s2, s3, s4) = state.coords;
    let mut coords = HashSet::new();
    coords.insert(s1);
    coords.insert(s2);
    coords.insert(s3);
    coords.insert(s4);
    coords
}

pub fn into_set(piece: Piece) -> HashSet<Coord> {
    unsafe {
        match piece {
            Piece { i } => { _into_set(i.state) }
            Piece { o } => { _into_set(o.state) }
            Piece { t } => { _into_set(t.state) }
            Piece { s } => { _into_set(s.state) }
            Piece { z } => { _into_set(z.state) }
            Piece { j } => { _into_set(j.state) }
            Piece { l } => { _into_set(l.state) }
        }
    }
}

const I_COLOR: [f32; 4] = [0.6, 0.8, 0.9, 1.0];
const O_COLOR: [f32; 4] = [0.9, 0.8, 0.0, 1.0];
const T_COLOR: [f32; 4] = [0.9, 0.2, 1.0, 1.0];
const S_COLOR: [f32; 4] = [0.6, 0.8, 0.5, 1.0];
const Z_COLOR: [f32; 4] = [0.7, 0.1, 0.2, 1.0];
const J_COLOR: [f32; 4] = [0.4, 0.1, 1.0, 1.0];
const L_COLOR: [f32; 4] = [0.9, 0.5, 0.0, 1.0];

pub fn piece_color(piece: Piece) -> [f32; 4] {
	unsafe {
		match piece {
            Piece { i } => { I_COLOR }
            Piece { o } => { O_COLOR }
            Piece { t } => { T_COLOR }
            Piece { s } => { S_COLOR }
            Piece { z } => { Z_COLOR }
            Piece { j } => { J_COLOR }
            Piece { l } => { L_COLOR }
		}
	}
}
