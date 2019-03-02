extern crate rand;

use std::collections::HashSet;
use tetris::piece::rand::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Copy)]
pub enum Piece {
    I(State),
    O(State),
    T(State),
    S(State),
    Z(State),
    J(State),
    L(State),
}

#[derive(Clone, Copy)]
pub struct State {
    coords: (Coord, Coord, Coord, Coord),
    position: u8,
}

fn new_i(width: i8) -> Piece {
    let coords = (
        Coord {
            x: width / 2,
            y: -2,
        },
        Coord {
            x: width / 2,
            y: -1,
        },
        Coord { x: width / 2, y: 0 },
        Coord { x: width / 2, y: 1 },
    );
    let state = State {
        coords: coords,
        position: 0,
    };
    Piece::I(state)
}

fn new_o(width: i8) -> Piece {
    let coords = (
        Coord {
            x: width / 2,
            y: -1,
        },
        Coord {
            x: width / 2 + 1,
            y: -1,
        },
        Coord { x: width / 2, y: 0 },
        Coord {
            x: width / 2 + 1,
            y: 0,
        },
    );
    let state = State {
        coords: coords,
        position: 0,
    };
    Piece::O(state)
}

fn new_t(width: i8) -> Piece {
    let coords = (
        Coord {
            x: width / 2,
            y: -1,
        },
        Coord {
            x: width / 2 - 1,
            y: 0,
        },
        Coord { x: width / 2, y: 0 },
        Coord {
            x: width / 2 + 1,
            y: 0,
        },
    );
    let state = State {
        coords: coords,
        position: 0,
    };
    Piece::T(state)
}

fn new_s(width: i8) -> Piece {
    let coords = (
        Coord {
            x: width / 2,
            y: -1,
        },
        Coord {
            x: width / 2 + 1,
            y: -1,
        },
        Coord {
            x: width / 2 - 1,
            y: 0,
        },
        Coord { x: width / 2, y: 0 },
    );
    let state = State {
        coords: coords,
        position: 0,
    };
    Piece::S(state)
}

fn new_z(width: i8) -> Piece {
    let coords = (
        Coord {
            x: width / 2 - 1,
            y: -1,
        },
        Coord {
            x: width / 2,
            y: -1,
        },
        Coord { x: width / 2, y: 0 },
        Coord {
            x: width / 2 + 1,
            y: 0,
        },
    );
    let state = State {
        coords: coords,
        position: 0,
    };
    Piece::Z(state)
}

fn new_j(width: i8) -> Piece {
    let coords = (
        Coord {
            x: width / 2 - 1,
            y: -1,
        },
        Coord {
            x: width / 2 - 1,
            y: 0,
        },
        Coord { x: width / 2, y: 0 },
        Coord {
            x: width / 2 + 1,
            y: 0,
        },
    );
    let state = State {
        coords: coords,
        position: 0,
    };
    Piece::J(state)
}

fn new_l(width: i8) -> Piece {
    let coords = (
        Coord {
            x: width / 2 + 1,
            y: -1,
        },
        Coord {
            x: width / 2 + 1,
            y: 0,
        },
        Coord { x: width / 2, y: 0 },
        Coord {
            x: width / 2 - 1,
            y: 0,
        },
    );
    let state = State {
        coords: coords,
        position: 0,
    };
    Piece::L(state)
}

pub fn random(width: i8) -> Piece {
    let r = rand::thread_rng().next_u32();
    match r % 7 {
        0 => new_i(width),
        1 => new_o(width),
        2 => new_t(width),
        3 => new_s(width),
        4 => new_z(width),
        5 => new_j(width),
        6 => new_l(width),
        _ => panic!("unreachable random piece generation"),
    }
}

fn _left(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    let coords = (
        Coord {
            x: s1.x - 1,
            y: s1.y,
        },
        Coord {
            x: s2.x - 1,
            y: s2.y,
        },
        Coord {
            x: s3.x - 1,
            y: s3.y,
        },
        Coord {
            x: s4.x - 1,
            y: s4.y,
        },
    );
    State {
        coords: coords,
        position: state.position,
    }
}

pub fn left(piece: Piece) -> Piece {
    match piece {
        Piece::I(state) => Piece::I(_left(state)),
        Piece::O(state) => Piece::O(_left(state)),
        Piece::T(state) => Piece::T(_left(state)),
        Piece::S(state) => Piece::S(_left(state)),
        Piece::Z(state) => Piece::Z(_left(state)),
        Piece::J(state) => Piece::J(_left(state)),
        Piece::L(state) => Piece::L(_left(state)),
    }
}

fn _right(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    let coords = (
        Coord {
            x: s1.x + 1,
            y: s1.y,
        },
        Coord {
            x: s2.x + 1,
            y: s2.y,
        },
        Coord {
            x: s3.x + 1,
            y: s3.y,
        },
        Coord {
            x: s4.x + 1,
            y: s4.y,
        },
    );
    State {
        coords: coords,
        position: state.position,
    }
}

pub fn right(piece: Piece) -> Piece {
    match piece {
        Piece::I(state) => Piece::I(_right(state)),
        Piece::O(state) => Piece::O(_right(state)),
        Piece::T(state) => Piece::T(_right(state)),
        Piece::S(state) => Piece::S(_right(state)),
        Piece::Z(state) => Piece::Z(_right(state)),
        Piece::J(state) => Piece::J(_right(state)),
        Piece::L(state) => Piece::L(_right(state)),
    }
}

fn _down(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    let coords = (
        Coord {
            x: s1.x,
            y: s1.y + 1,
        },
        Coord {
            x: s2.x,
            y: s2.y + 1,
        },
        Coord {
            x: s3.x,
            y: s3.y + 1,
        },
        Coord {
            x: s4.x,
            y: s4.y + 1,
        },
    );
    State {
        coords: coords,
        position: state.position,
    }
}

pub fn down(piece: Piece) -> Piece {
    match piece {
        Piece::I(state) => Piece::I(_down(state)),
        Piece::O(state) => Piece::O(_down(state)),
        Piece::T(state) => Piece::T(_down(state)),
        Piece::S(state) => Piece::S(_down(state)),
        Piece::Z(state) => Piece::Z(_down(state)),
        Piece::J(state) => Piece::J(_down(state)),
        Piece::L(state) => Piece::L(_down(state)),
    }
}

fn rotate_i(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    if state.position == 0 {
        let coords = (
            Coord {
                x: s1.x - 2,
                y: s1.y + 2,
            },
            Coord {
                x: s2.x - 1,
                y: s2.y + 1,
            },
            s3,
            Coord {
                x: s4.x + 1,
                y: s4.y - 1,
            },
        );
        State {
            coords: coords,
            position: 0,
        }
    } else {
        let coords = (
            Coord {
                x: s1.x + 2,
                y: s1.y - 2,
            },
            Coord {
                x: s2.x + 1,
                y: s2.y - 1,
            },
            s3,
            Coord {
                x: s4.x - 1,
                y: s4.y + 1,
            },
        );
        State {
            coords: coords,
            position: 1,
        }
    }
}

fn rotate_right_t(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    match state.position {
        0 => {
            let coords = (
                Coord {
                    x: s1.x - 1,
                    y: s1.y + 1,
                },
                Coord {
                    x: s2.x + 1,
                    y: s2.y + 1,
                },
                s3,
                Coord {
                    x: s4.x - 1,
                    y: s4.y - 1,
                },
            );
            State {
                coords: coords,
                position: 1,
            }
        }
        1 => {
            let coords = (
                Coord {
                    x: s1.x + 1,
                    y: s1.y + 1,
                },
                Coord {
                    x: s2.x + 1,
                    y: s2.y - 1,
                },
                s3,
                Coord {
                    x: s4.x - 1,
                    y: s4.y + 1,
                },
            );
            State {
                coords: coords,
                position: 2,
            }
        }
        2 => {
            let coords = (
                Coord {
                    x: s1.x + 1,
                    y: s1.y - 1,
                },
                Coord {
                    x: s2.x - 1,
                    y: s2.y - 1,
                },
                s3,
                Coord {
                    x: s4.x + 1,
                    y: s4.y + 1,
                },
            );
            State {
                coords: coords,
                position: 3,
            }
        }
        3 => {
            let coords = (
                Coord {
                    x: s1.x - 1,
                    y: s1.y - 1,
                },
                Coord {
                    x: s2.x - 1,
                    y: s2.y + 1,
                },
                s3,
                Coord {
                    x: s4.x + 1,
                    y: s4.y - 1,
                },
            );
            State {
                coords: coords,
                position: 0,
            }
        }
        _ => panic!("Invalid T position"),
    }
}

fn rotate_s(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    if state.position == 0 {
        let coords = (
            Coord {
                x: s1.x - 1,
                y: s1.y - 1,
            },
            s2,
            Coord {
                x: s3.x + 1,
                y: s3.y - 1,
            },
            Coord {
                x: s4.x + 2,
                y: s4.y,
            },
        );
        State {
            coords: coords,
            position: 1,
        }
    } else {
        let coords = (
            Coord {
                x: s1.x + 1,
                y: s1.y + 1,
            },
            s2,
            Coord {
                x: s3.x - 1,
                y: s3.y + 1,
            },
            Coord {
                x: s4.x - 2,
                y: s4.y,
            },
        );
        State {
            coords: coords,
            position: 0,
        }
    }
}

fn rotate_z(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    if state.position == 0 {
        let coords = (
            Coord {
                x: s1.x + 1,
                y: s1.y + 1,
            },
            s2,
            Coord {
                x: s3.x - 1,
                y: s3.y + 1,
            },
            Coord {
                x: s4.x - 2,
                y: s4.y,
            },
        );
        State {
            coords: coords,
            position: 1,
        }
    } else {
        let coords = (
            Coord {
                x: s1.x - 1,
                y: s1.y - 1,
            },
            s2,
            Coord {
                x: s3.x + 1,
                y: s3.y - 1,
            },
            Coord {
                x: s4.x + 2,
                y: s4.y,
            },
        );
        State {
            coords: coords,
            position: 0,
        }
    }
}

fn rotate_right_j(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    match state.position {
        0 => {
            let coords = (
                Coord {
                    x: s1.x - 1,
                    y: s1.y + 1,
                },
                s2,
                Coord {
                    x: s3.x - 1,
                    y: s3.y - 1,
                },
                Coord {
                    x: s4.x - 2,
                    y: s4.y - 2,
                },
            );
            State {
                coords: coords,
                position: 1,
            }
        }
        1 => {
            let coords = (
                Coord {
                    x: s1.x + 1,
                    y: s1.y + 1,
                },
                s2,
                Coord {
                    x: s3.x - 1,
                    y: s3.y + 1,
                },
                Coord {
                    x: s4.x - 2,
                    y: s4.y + 2,
                },
            );
            State {
                coords: coords,
                position: 2,
            }
        }
        2 => {
            let coords = (
                Coord {
                    x: s1.x + 1,
                    y: s1.y - 1,
                },
                s2,
                Coord {
                    x: s3.x + 1,
                    y: s3.y + 1,
                },
                Coord {
                    x: s4.x + 2,
                    y: s4.y + 2,
                },
            );
            State {
                coords: coords,
                position: 3,
            }
        }
        3 => {
            let coords = (
                Coord {
                    x: s1.x - 1,
                    y: s1.y - 1,
                },
                s2,
                Coord {
                    x: s3.x + 1,
                    y: s3.y - 1,
                },
                Coord {
                    x: s4.x + 2,
                    y: s4.y - 2,
                },
            );
            State {
                coords: coords,
                position: 0,
            }
        }
        _ => panic!("Invalid J position"),
    }
}

fn rotate_right_l(state: State) -> State {
    let (s1, s2, s3, s4) = state.coords;
    match state.position {
        0 => {
            let coords = (
                Coord {
                    x: s1.x - 1,
                    y: s1.y + 1,
                },
                s2,
                Coord {
                    x: s3.x + 1,
                    y: s3.y + 1,
                },
                Coord {
                    x: s4.x + 2,
                    y: s4.y + 2,
                },
            );
            State {
                coords: coords,
                position: 1,
            }
        }
        1 => {
            let coords = (
                Coord {
                    x: s1.x + 1,
                    y: s1.y + 1,
                },
                s2,
                Coord {
                    x: s3.x + 1,
                    y: s3.y - 1,
                },
                Coord {
                    x: s4.x + 2,
                    y: s4.y - 2,
                },
            );
            State {
                coords: coords,
                position: 2,
            }
        }
        2 => {
            let coords = (
                Coord {
                    x: s1.x + 1,
                    y: s1.y - 1,
                },
                s2,
                Coord {
                    x: s3.x - 1,
                    y: s3.y - 1,
                },
                Coord {
                    x: s4.x - 2,
                    y: s4.y - 2,
                },
            );
            State {
                coords: coords,
                position: 3,
            }
        }
        3 => {
            let coords = (
                Coord {
                    x: s1.x - 1,
                    y: s1.y - 1,
                },
                s2,
                Coord {
                    x: s3.x - 1,
                    y: s3.y + 1,
                },
                Coord {
                    x: s4.x - 2,
                    y: s4.y + 2,
                },
            );
            State {
                coords: coords,
                position: 0,
            }
        }
        _ => panic!("Invalid L position"),
    }
}

pub fn rotate_right(piece: Piece) -> Piece {
    match piece {
        Piece::I(state) => Piece::I(rotate_i(state)),
        Piece::O(state) => Piece::O(state),
        Piece::T(state) => Piece::T(rotate_right_t(state)),
        Piece::S(state) => Piece::S(rotate_s(state)),
        Piece::Z(state) => Piece::Z(rotate_z(state)),
        Piece::J(state) => Piece::J(rotate_right_j(state)),
        Piece::L(state) => Piece::L(rotate_right_l(state)),
    }
}

fn _into_set(state: &State) -> HashSet<Coord> {
    let (s1, s2, s3, s4) = state.coords;
    let mut coords = HashSet::new();
    coords.insert(s1);
    coords.insert(s2);
    coords.insert(s3);
    coords.insert(s4);
    coords
}

pub fn into_set(piece: &Piece) -> HashSet<Coord> {
    match piece {
        Piece::I(state) => _into_set(state),
        Piece::O(state) => _into_set(state),
        Piece::T(state) => _into_set(state),
        Piece::S(state) => _into_set(state),
        Piece::Z(state) => _into_set(state),
        Piece::J(state) => _into_set(state),
        Piece::L(state) => _into_set(state),
    }
}
