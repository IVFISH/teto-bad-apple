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
        self.action(Box::new(PieceMove::new (0, -1)))
    }

    pub fn move_right(&mut self) -> bool {
        self.action(Box::new(PieceMove::new(0, 1)))
    }

    pub fn soft_drop(&mut self) -> bool {
        self.action(Box::new(SoftDrop::new()))
    }

    pub fn rotate_cw(&mut self) -> bool {
        self.action(Box::new(PieceRotate::new(1)))
    }

    pub fn rotate_180(&mut self) -> bool {
        self.action(Box::new(PieceRotate::new(2)))
    }

    pub fn rotate_ccw(&mut self) -> bool {
        self.action(Box::new(PieceRotate::new(3)))
    }

    pub fn hard_drop(&mut self) -> bool {
        let sd = Box::new(SoftDrop::new());
        let set = Box::new(SetPiece::new());
        let nxt = Box::new(NextPiece::new(self.game.active));
        let commands = Batch {commands: vec![sd, set, nxt]};

        self.action(Box::new(commands))
    }

    fn action(&mut self, command: Box<dyn Command>) -> bool {
        self.stack.push_front(command);
        self.stack.get_mut(0).unwrap().execute(&mut self.game)
    }

}