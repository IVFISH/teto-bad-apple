#![allow(dead_code)]

use crate::board::*;
use crate::piece::*;
use crate::queue::*;
use std::fmt::{Display, Formatter};

pub struct Game {
    pub board: Board,
    pub queue: Queue,
    pub active: Placement,
    pub hold: Option<usize>,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue; {}\n", self.queue)?;
        write!(f, "Hold: {:?}\n", self.hold)?;
        write!(f, "{}", self.board.to_string(&self.active))
    }
}

impl Game {
    pub fn new(height: usize, width: usize, seed: usize) -> Self {
        let mut queue = Queue::new(seed);
        let board = Board::new(width, height);
        let active = new_piece(queue.next(), height, width);

        Self {
            board,
            queue,
            active,
            hold: None,
        }
    }

    pub fn new_piece(&self, piece_type: usize) -> Placement {
        new_piece(piece_type, self.board.height, self.board.width)
    }
}

fn default_piece_spawn(height: usize, width: usize) -> (i8, i8) {
    (height as i8 - 3, width as i8 / 2 - 1)
}

pub fn new_piece(piece_type: usize, height: usize, width: usize) -> Placement {
    let (row, col) = default_piece_spawn(height, width);
    Placement::new(piece_type, 0, row, col)
}
