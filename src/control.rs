#![allow(dead_code)]

use std::collections::VecDeque;
use crate::piece::Point;
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

impl<F> PieceMove<F> where F: Fn() -> (i8, i8) {
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
            game.active.shift(-y, -x);
        };

        self.moved
    }

    fn undo(&mut self, game: &mut Game) {
        if self.moved {
            game.active.shift(0, 1);
        }
    }
}

pub struct PieceRotate {
    direction: usize,
    rotated: bool,
    kick: Point
}

impl PieceRotate {
    pub fn new(direction: usize) -> Self {
        Self {
            direction,
            rotated: false,
            kick: [0, 0]
        }
    }
}

impl Command for PieceRotate {
    fn execute(&mut self, game: &mut Game) -> bool {
        game.active.rotate(self.direction);

        for [y, x] in game.active.get_offsets(self.direction) {
            let mut command = PieceMove::new(Box::new(|| (y, x)));
            if command.execute(game) {
                self.kick = [y, x];
                self.rotated = true;
                return true;
            }
        }

        game.active.rotate(4 - self.direction);
        false
    }

    fn undo(&mut self, game: &mut Game) {
        if self.rotated {
            let [y, x] = self.kick;
            game.active.shift(-y, -x);
            game.active.rotate(4 - self.direction);
        }
    }
}

// pub struct HardDrop {
//
// }
//
// impl HardDrop {
//
// }
//
// impl Command for HardDrop {
//     fn execute(&mut self, game: &mut Game) -> bool {
//         todo!()
//     }
//
//     fn undo(&mut self, game: &mut Game) {
//         todo!()
//     }
// }

// pub fn set_piece(&mut self) {
//     let (row, col) = (self.active.row, self.active.col);
//     for [r, c] in self.active.rel_locations() {
//         self.board.add((r + row) as usize, (c + col) as usize);
//     }
// }