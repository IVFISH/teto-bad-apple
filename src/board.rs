#![allow(dead_code)]
use std::fmt::{Display, Formatter};

pub struct Board {
    arr: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(arr: Vec<Vec<bool>>) -> Self {
        let height = arr.len();
        let width = arr[0].len();
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
        let rows: Vec<usize> = self.arr
            .iter()
            .enumerate()
            .filter(|(_, row)|
                row.iter().all(|x| *x))
            .map(|(r, _)| r)
            .collect();

        for row in rows {
            self.line_clear(row);
        }
    }

    pub fn set_piece(piece: usize) {}

    pub fn piece_collision(&self, piece: usize) -> bool {
        false
    }

    pub fn piece_in_bounds(&self, piece: usize) -> bool {
        false
    }

    pub fn piece_grounded(&self, piece: usize) -> bool {
        false
    }

    pub fn piece_valid_location(&self, piece: usize) -> bool {
        !self.piece_collision(piece) && self.piece_in_bounds(piece)
    }

    pub fn piece_valid_placement(&self, piece: usize) -> bool {
        self.piece_valid_location(piece) && self.piece_grounded(piece)
    }

    fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.width && col < self.height
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
