#![allow(dead_code)]
use crate::piece::*;
use std::fmt::{Display, Formatter};

pub struct Board {
    arr: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn from_vec(arr: Vec<Vec<bool>>) -> Self {
        let height = arr.len();
        let width = arr[0].len();
        Self { arr, width, height }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let arr = vec![vec![false; height]; width];
        Self { arr, width, height }
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        self.arr[row][col]
    }

    pub fn add(&mut self, row: usize, col: usize) {
        self.arr[row][col] = true;
    }

    pub fn remove(&mut self, row: usize, col: usize) {
        self.arr[row][col] = false;
    }

    pub fn line_clear(&mut self, row: usize) {
        self.arr.remove(row);
        self.arr.push(vec![false; self.width])
    }

    pub fn clear_all_lines(&mut self) {
        let rows: Vec<usize> = self
            .arr
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|x| *x))
            .map(|(r, _)| r)
            .collect();

        for row in rows {
            self.line_clear(row);
        }
    }

    pub fn piece_collision(&self, piece: &Placement) -> bool {
        piece
            .rel_locations()
            .iter()
            .all(|[r, c]| !self.get((r + piece.row) as usize, (c + piece.col) as usize))
    }

    pub fn piece_in_bounds(&self, piece: &Placement) -> bool {
        piece.rel_locations().iter().all(|[r, c]| {
            ((r + piece.row) as usize) < self.width && ((c + piece.col) as usize) < self.height
        })
    }

    pub fn piece_valid_location(&self, piece: &Placement) -> bool {
        !self.piece_collision(piece) && self.piece_in_bounds(piece)
    }

    pub fn piece_grounded(&self, piece: &Placement) -> bool {
        let moved = &Placement::new(
            piece.piece_type,
            piece.rotation_state,
            piece.row - 1,
            piece.col,
        );
        !self.piece_valid_location(moved)
    }

    pub fn piece_valid_placement(&self, piece: &Placement) -> bool {
        self.piece_valid_location(piece) && self.piece_grounded(piece)
    }

    fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.width && col < self.height
    }

    pub fn to_string(&self, piece: &Placement) -> String {
        let mut out = String::new();
        let locations = piece.rel_locations();

        for row in (0..self.height).rev() {
            for col in 0..self.width {
                if self.get(row, col)
                    || locations.contains(&[row as i8 + piece.row, col as i8 + piece.col])
                {
                    out.push_str("■ ");
                } else {
                    out.push_str("□ ");
                }
            }
            out.push_str("\n");
        }
        out
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in (0..self.height).rev() {
            for col in 0..self.width {
                if self.get(row, col) {
                    write!(f, "■ ")?
                } else {
                    write!(f, "□ ")?
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}
