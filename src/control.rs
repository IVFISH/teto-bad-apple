#![allow(dead_code)]

use std::collections::VecDeque;
use crate::game::*;

pub enum Input {
    SoftDrop,
    HardDrop,
    MoveLeft,
    MoveRight,
    RotateCW,
    Rotate180,
    RotateCCW
}

pub trait Command {
    fn execute(&mut self, game: &mut Game) -> bool;
    fn undo(&mut self, game: &mut Game);
}

pub struct PieceMove<F> where F: Fn() -> (i8, i8) {
    moved: bool,
    dir_gen: F
}

impl<'a, F> PieceMove<F> where F: Fn() -> (i8, i8) {
    pub fn new(dir_gen: F) -> Self {
        Self {
            moved: false,
            dir_gen
        }
    }
}

impl<F> Command for PieceMove<F> where F: Fn() -> (i8, i8) {
    fn execute(&mut self, game: &mut Game) -> bool {
        let (y, x) = (self.dir_gen)();
        game.active.shift(y, x);
        self.moved = game.board.piece_valid_location(&game.active);
        if !self.moved {
            game.active.shift(0, 1);
        };

        self.moved
    }

    fn undo(&mut self, game: &mut Game) {
        println!("{}", self.moved);
        if self.moved {
            game.active.shift(0, 1);
        }
    }
}

// pub struct PieceLeft<'a> {
//     game: &'a mut Game,
//     moved: bool
// }
//
// impl<'a> PieceLeft<'a> {
//     pub fn new(game: &'a mut Game) -> Self {
//         Self {
//             game,
//             moved: false
//         }
//     }
// }
//
// impl Command for PieceLeft<'_> {
//     fn execute(&mut self) -> bool {
//         self.game.active.shift(0, -1);
//         self.moved = self.game.board.piece_valid_location(&self.game.active);
//
//         if !self.moved {
//             self.game.active.shift(0, 1);
//         };
//
//         self.moved
//     }
//
//     fn undo(&mut self) {
//         if self.moved {
//             self.game.active.shift(0, 1);
//         }
//     }
// }
//
// pub struct PieceRight<'a> {
//     game: &'a mut Game,
//     moved: bool
// }
//
// impl PieceRight<'_> {
//     pub fn new(game: &mut Game) -> Self {
//         Self {
//             game,
//             moved: false
//         }
//     }
// }
//
// impl Command for PieceRight<'_> {
//     fn execute(&mut self) -> bool {
//         self.game.active.shift(0, 1);
//         self.moved = self.game.board.piece_valid_location(&self.game.active);
//
//         if !self.moved {
//             self.game.active.shift(0, -1);
//         };
//
//         self.moved
//     }
//
//     fn undo(&mut self) {
//         if self.moved {
//             self.game.active.shift(0, 1);
//         }
//     }
// }

// pub fn piece_drop(&mut self) {
//     while self.piece_down() {}
// }
//
// pub fn piece_right(&mut self) -> bool {false}
//
// pub fn piece_left(&mut self) -> bool {false}
//
// pub fn safe_cw(&mut self) -> bool {false}
//
// pub fn safe_180(&mut self) -> bool {false}
//
// pub fn safe_ccw(&mut self) -> bool {false}

// pub fn hd(&mut self) {
//     self.piece_down();
//     self.set_piece();
//     let next = self.queue.next();
//     self.active = self.new_piece(next);
// }
//
// pub fn set_piece(&mut self) {
//     let (row, col) = (self.active.row, self.active.col);
//     for [r, c] in self.active.rel_locations() {
//         self.board.add((r + row) as usize, (c + col) as usize);
//     }
// }