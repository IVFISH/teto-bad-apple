#![allow(dead_code)]

pub type Point = [i8; 2];
use data::PIECES;

#[derive(Copy, Clone, Debug)]
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

    pub fn rel_locations(&self) -> [Point; 4] {
        PIECES[self.piece_type][self.rotation_state]
    }

    pub fn shift(&mut self, y: i8, x: i8) {
        self.row += y;
        self.col += x;
    }

    pub fn rotate(&mut self, direction: usize) {
        self.rotation_state = (self.rotation_state + direction) % 4;
    }
}

mod data {
    use super::Point;
    use offsets::*;
    use rotations::*;

    pub const ROTATION_STATES: usize = 4;
    pub const NUM_ROTATION_DIRECTIONS: usize = 3;
    pub const PIECE_SIZE: usize = 4;
    pub const NUM_PIECES: usize = 7;

    pub type PieceLocations = [[Point; PIECE_SIZE]; ROTATION_STATES];

    pub const PIECES: [PieceLocations; NUM_PIECES] = [Z, L, O, S, I, J, T];
    pub const OFFSETS: [PieceOffsets; NUM_PIECES] = [Z_OFFSET; NUM_PIECES];

    mod rotations {
        use super::PieceLocations;

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
        use super::*;

        pub type Offset = [Point; 6];
        pub type PieceOffsets = [Offset; NUM_ROTATION_DIRECTIONS];

        pub const Z_OFFSET: [Offset; NUM_ROTATION_DIRECTIONS] = [[[0, 0]; 6]; 3];
    }
}
