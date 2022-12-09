#![allow(dead_code)]

use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::game::*;
use crate::control::*;

pub struct Bot {
    game: Game,
    stack: VecDeque<Box<dyn Command>>
}

impl Display for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.game)?;
        Ok(())
    }
}

impl Bot {
    pub fn new(height: usize, width: usize, seed: usize) -> Self {
        Self {
            game: Game::new(height, width, seed),
            stack: VecDeque::new()
        }
    }

    pub fn undo(&mut self) {
        let mut command = self.stack.pop_front().unwrap();
        command.undo(&mut self.game);
    }

    pub fn move_left(&mut self) -> bool {
        self.piece_move(Box::new(|| (0, -1)))
    }

    pub fn move_right(&mut self) -> bool {
        self.piece_move(Box::new(|| (0, 1)))
    }

    pub fn soft_drop(&mut self) {
        while self.down() {}
    }

    fn down(&mut self) -> bool {
        self.piece_move(Box::new(|| (-1, 0)))
    }

    fn piece_move(&mut self, direction: Box<dyn Fn() -> (i8, i8)>) -> bool {
        let command = Box::new(PieceMove::new(direction));
        self.stack.push_front(command);
        self.stack.get_mut(0).unwrap().execute(&mut self.game)
    }

    pub fn rotate_cw(&mut self) -> bool {
        self.piece_rotate(1)
    }

    pub fn rotate_180(&mut self) -> bool {
        self.piece_rotate(2)
    }

    pub fn rotate_ccw(&mut self) -> bool {
        self.piece_rotate(3)
    }

    fn piece_rotate(&mut self, direction: usize) -> bool {
        let command = Box::new(PieceRotate::new(direction));
        self.stack.push_front(command);
        self.stack.get_mut(0).unwrap().execute(&mut self.game)
    }

}