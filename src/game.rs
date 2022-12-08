#![allow(dead_code)]

use crate::board::*;
use crate::piece::*;
use crate::queue::*;
use std::fmt::{Display, Formatter};

pub struct Game {
    pub board: Board,
    pub queue: Queue,
    pub active: Placement,
    pub hold: Option<Placement>,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue; {}", self.queue)?;
        write!(f, "{}", self.board)
    }
}

impl Game {
    pub fn new(&mut self, width: usize, height: usize, seed: usize) -> Self {
        let mut queue = Queue::new(seed);
        let board = Board::new(width, height);
        let active = self.new_piece(queue.next());

        Self {
            board,
            queue,
            active,
            hold: None,
        }
    }

    pub fn new_piece(&self, piece_type: usize) -> Placement {
        let (row, col) = default_piece_spawn(self.board.width, self.board.height);
        Placement::new(piece_type, 0, row, col)
    }

    pub fn hd(&mut self) {
        self.piece_down();
        self.set_piece();
        let next = self.queue.next();
        self.active = self.new_piece(next);
    }

    pub fn set_piece(&mut self) {
        let (row, col) = (self.active.row, self.active.col);
        for [r, c] in self.active.rel_locations() {
            self.board.add((r + row) as usize, (c + col) as usize);
        }
    }
}

fn default_piece_spawn(width: usize, height: usize) -> (i8, i8) {
    (width as i8 / 2, height as i8 - 2)
}
