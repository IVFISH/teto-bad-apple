#![allow(dead_code)]

pub type Point = [i8; 2];
use data::{PIECES, OFFSETS};

#[derive(Copy, Clone, Debug)]
pub struct Placement {
    pub piece_type: usize,
    pub rotation_state: usize,
    pub row: i8,
    pub col: i8,
}

impl Placement {
    fn new(piece_type: usize, rotation_state: usize, row: i8, col: i8) -> Self {
        Self {
            piece_type,
            rotation_state,
            row,
            col,
        }
    }

    fn rel_locations(&self) -> [Point; 4] {
        PIECES[self.piece_type][self.rotation_state]
    }

    fn abs_locations(&self) -> [Point; 4] {
        let offset = [];

        [[0, 0]; 4]
    }

    fn shift(&mut self, y: i8, x: i8) {
        self.row += y;
        self.col += x;
    }

    fn rotate(&mut self, direction: usize) {
        self.rotation_state = (self.rotation_state + direction) % 4;
    }
}

fn point_add(p1: Point, p2: Point) -> Point {
    [p1[0] + p2[0], p1[1] + p2[1]]
}

mod data {
    use rotations::*;
    use offsets::*;
    use super::Point;

    pub const ROTATION_STATES: usize = 4;
    pub const PIECE_SIZE: usize = 4;
    pub const NUM_PIECES: usize = 7;

    pub type PieceLocations = [[Point; PIECE_SIZE]; ROTATION_STATES];

    pub const PIECES: [PieceLocations; NUM_PIECES] = [Z, L, O, S, I, J, T];
    pub const OFFSETS: [PieceLocations; NUM_PIECES] = [Z, L, O, S, I, J, T];

    mod rotations {
        use super::PieceLocations;

        pub const Z: PieceLocations = [
            [[1, -1], [1, 0], [0, 0], [0, 1]],
            [[1, 1], [0, 1], [0, 0], [-1, 0]],
            [[-1, 1], [-1, 0], [0, 0], [0, -1]],
            [[-1, -1], [0, -1], [0, 0], [1, 0]]
        ];

        pub const L: PieceLocations = [
            [[1, 1], [0, -1], [0, 0], [0, 1]],
            [[-1, 1], [1, 0], [0, 0], [-1, 0]],
            [[-1, -1], [0, 1], [0, 0], [0, -1]],
            [[1, -1], [-1, 0], [0, 0], [1, 0]],
        ];

        pub const O: PieceLocations = [
            [[1, 0], [1, 1], [0, 0], [0, 1]],
            [[0, 1], [-1, 1], [0, 0], [-1, 0]],
            [[-1, 0], [-1, -1], [0, 0], [0, -1]],
            [[0, -1], [1, -1], [0, 0], [1, 0]],
        ];

        pub const S: PieceLocations = [
            [[1, 0], [1, 1], [0, -1], [0, 0]],
            [[0, 1], [-1, 1], [1, 0], [0, 0]],
            [[-1, 0], [-1, -1], [0, 1], [0, 0]],
            [[0, -1], [1, -1], [-1, 0], [0, 0]],
        ];

        pub const I: PieceLocations = [
            [[0, -1], [0, 0], [0, 1], [0, 2]],
            [[1, 0], [0, 0], [-1, 0], [-2, 0]],
            [[0, 1], [0, 0], [0, -1], [0, -2]],
            [[-1, 0], [0, 0], [1, 0], [2, 0]],
        ];

        pub const J: PieceLocations = [
            [[1, -1], [0, -1], [0, 0], [0, 1]],
            [[1, 1], [1, 0], [0, 0], [-1, 0]],
            [[-1, 1], [0, 1], [0, 0], [0, -1]],
            [[-1, -1], [-1, 0], [0, 0], [1, 0]],
        ];

        pub const T: PieceLocations = [
            [[1, 0], [0, -1], [0, 0], [0, 1]],
            [[0, 1], [1, 0], [0, 0], [-1, 0]],
            [[-1, 0], [0, 1], [0, 0], [0, -1]],
            [[0, -1], [-1, 0], [0, 0], [1, 0]],
        ];
    }

    mod offsets {
        use super::PieceLocations;

        // some have 6 but fuck that
        pub type OFFSET = [PieceLocations; 5];
    }
}



