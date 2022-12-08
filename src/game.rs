#![allow(dead_code)]

use crate::board::*;
use crate::piece::*;
use std::fmt::{Display, Formatter};
use std::collections::VecDeque;

pub struct Game {
    pub board: Board,
    pub queue: Queue,
    pub active: Placement,
    pub hold: Option<Placement>,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue; {}", self.queue)?;
        write!(f, "{}", self.board)?;

        Ok(())
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

    pub fn piece_down(&mut self) -> bool {false}

    pub fn piece_drop(&mut self) {
        while self.piece_down() {}
    }

    pub fn piece_right(&mut self) -> bool {false}

    pub fn piece_left(&mut self) -> bool {false}

    pub fn safe_cw(&mut self) -> bool {false}

    pub fn safe_180(&mut self) -> bool {false}

    pub fn safe_ccw(&mut self) -> bool {false}
}

fn default_piece_spawn(width: usize, height: usize) -> (i8, i8) {
    (width as i8 / 2, height as i8)
}

pub struct Queue {
    seed: usize,
    pieces: VecDeque<usize>,
    a: usize,
    m: usize,
}

impl Display for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.pieces)
    }
}

impl Queue {
    pub fn new(seed: usize) -> Self {
        Self {seed, pieces: VecDeque::new(), a: 16807, m: 2147483647}
    }

    pub fn next(&mut self) -> usize {
        if self.pieces.len() < 10 {
           self.seven_bag()
        }

        self.pieces.pop_front().unwrap()
    }

    fn next_num(&mut self) -> f32 {
        self.seed = self.a * self.seed % self.m;

        let out = (self.seed - 1) as f32 / self.m as f32;
        out
    }

    fn seven_bag(&mut self) {
        let mut arr = [0, 1, 2, 3, 4, 5, 6];
        for i in (1..7).rev() {
            let r = (self.next_num() * (i as f32 + 1.0)) as usize;
            (arr[i], arr[r]) = (arr[r], arr[i])
        }
        self.pieces.extend(arr.iter());
    }
}
