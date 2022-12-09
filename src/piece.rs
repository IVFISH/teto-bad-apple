#![allow(dead_code)]

pub type Point = [i8; 2];

use data::{PIECES, OFFSETS, PieceLocation, Offset};
use crate::piece::data::PieceLocations;

#[derive(Copy, Clone, Debug, Default)]
pub struct Placement {
    pub piece_type: usize,
    pub rotation_state: usize,
    pub row: i8,
    pub col: i8,
}

impl Placement {
    pub fn new(piece_type: usize, rotation_state: usize, row: i8, col: i8) -> Self {
        Self {
            piece_type,
            rotation_state,
            row,
            col,
        }
    }

    pub fn rel_locations(&self) -> PieceLocation {
        PIECES[self.piece_type][self.rotation_state]
    }

    pub fn shift(&mut self, y: i8, x: i8) {
        self.row += y;
        self.col += x;
    }

    pub fn rotate(&mut self, direction: usize) {
        self.rotation_state = (self.rotation_state + direction) % 4;
    }

    pub fn get_offsets(&self, direction: usize) -> Offset {
        OFFSETS[self.piece_type][self.rotation_state][direction - 1]
    }
}

mod data {
    pub use offsets::*;
    pub use rotations::*;
    use super::Point;

    pub const ROTATION_STATES: usize = 4;
    pub const NUM_ROTATION_DIRECTIONS: usize = 3;
    pub const PIECE_SIZE: usize = 4;
    pub const NUM_PIECES: usize = 7;

    mod rotations {
        use super::*;

        pub const PIECES: [PieceLocations; NUM_PIECES] = [Z, L, O, S, I, J, T];
        pub type PieceLocation = [Point; PIECE_SIZE];
        pub type PieceLocations = [PieceLocation; ROTATION_STATES];

        pub const Z: PieceLocations = [
            [[1, -1], [1, 0], [0, 0], [0, 1]],
            [[1, 1], [0, 1], [0, 0], [-1, 0]],
            [[-1, 1], [-1, 0], [0, 0], [0, -1]],
            [[-1, -1], [0, -1], [0, 0], [1, 0]],
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
        // note: can try with lazy-static and vectors

        use super::*;

        pub const OFFSETS: [PieceOffsets; NUM_PIECES] = [G, G, O, G, I, G, G];
        pub type Offset = [Point; 6];
        pub type PieceOffsets = [[Offset; NUM_ROTATION_DIRECTIONS]; ROTATION_STATES];

        pub const G: PieceOffsets = [
            [
                [[0, 0], [0, -1], [1, -1], [-2, 0], [-2, -1], [-2, -1]],
                [[0, 0], [1, 0], [1, 1], [1, -1], [0, 1], [0, -1]],
                [[0, 0], [0, 1], [1, 1], [-2, 0], [-2, 1], [-2, -1]]
            ],
            [
                [[0, 0], [0, 1], [-1, 1], [2, 0], [2, 1], [2, 1]],
                [[0, 0], [0, 1], [2, 1], [1, 1], [2, 0], [-1, 0]],
                [[0, 0], [0, 1], [-1, 1], [2, 0], [2, 1], [2, 1]],
            ],
            [
                [[0, 0], [0, 1], [1, 1], [-2, 0], [-2, 1], [-2, 1]],
                [[0, 0], [-1, 0], [-1, -1], [-1, 1], [0, -1], [0, 1]],
                [[0, 0], [0, -1], [1, -1], [-2, 0], [-2, -1], [-2, -1]],
            ],
            [
                [[0, 0], [0, -1], [-1, -1], [2, 0], [2, -1], [2, -1]],
                [[0, 0], [0, -1], [2, -1], [1, -1], [2, 0], [-1, 0]],
                [[0, 0], [0, -1], [-1, -1], [2, 0], [2, -1], [2, -1]],
            ]
        ];

        pub const O: PieceOffsets = [
            [
                [[1, 0]; 6],
                [[1, 1]; 6],
                [[0, 1]; 6]
            ],
            [
                [[0, 1]; 6],
                [[-1, 1]; 6],
                [[-1, 0]; 6]
            ],
            [
                [[-1, 0]; 6],
                [[-1, -1]; 6],
                [[0, -1]; 6]
            ],
            [
                [[0, -1]; 6],
                [[1, -1]; 6],
                [[1, 0]; 6]
            ],
        ];

        pub const I: PieceOffsets = [
            [
                [[0, 1], [0, 2], [0, -1], [-1, -1], [2, 2], [2, 2]],
                [[-1, 1], [0, 1], [0, 1], [0, 1], [0, 1], [0, 1]],
                [[-1, 0], [-1, -1], [-1, 2], [-2, 2], [2, -1], [2, -1]]
            ],
            [
                [[-1, 0], [-1, -1], [-1, 2], [1, -1], [-2, 2], [-2, 2]],
                [[-1, -1], [-1, 0], [-1, 0], [-1, 0], [-1, 0], [-1, 0]],
                [[0, -1], [0, -2], [0, 1], [-2, -2], [1, 1], [1, 1]]
            ],
            [
                [[0, -1], [0, 1], [0, -2], [1, 1], [-2, -2], [-2, -2]],
                [[1, -1], [0, -1], [0, -1], [0, -1], [0, -1], [0, -1]],
                [[1, 0], [1, -2], [1, 1], [2, -2], [-1, 1], [-1, 1]]
            ],
            [
                [[1, 0], [1, 1], [1, -2], [-1, 1], [2, -2], [2, -2]],
                [[1, 1], [1, 0], [1, 0], [1, 0], [1, 0], [1, 0]],
                [[0, 1], [0, 2], [0, -1], [2, 2], [-1, -1], [-1, -1]]
            ]
        ];
    }
}
