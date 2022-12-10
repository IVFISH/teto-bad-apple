#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use crate::board::Board;
use crate::game::*;
use crate::control::*;
use crate::piece::Placement;

pub struct ComparablePlacement<'a> {
    pub placement: Placement,
    pub board: &'a Board,
    pub row: i8,
}

impl Debug for ComparablePlacement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.placement)
    }
}


impl PartialEq<Self> for ComparablePlacement<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.placement.eq(&other.placement)
    }
}

impl PartialOrd for ComparablePlacement<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        score(&self.placement, &self.board, self.row)
            .partial_cmp(&score(&other.placement, &other.board, other.row))
    }
}

fn score(piece: &Placement, board: &Board, row: i8) -> i8 {
    let mut out = 0;
    for [y, x] in piece.rel_locations() {
        let y = y + piece.row;
        let x = x + piece.col ;
        if board.get(y as usize, x as usize) {
            if row == y {
                out += 3;
            } else {
                out -= 2;
            }
        } else {
            out -= 20;
        }
    }
    out
}


pub struct Bot {
    pub game: Game,
    stack: VecDeque<Command>,
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
            stack: VecDeque::new(),
        }
    }

    pub fn search(&mut self, used: &mut HashSet<Placement>) {
        let commands: Vec<Command> = vec![
            PieceMove::new(0, -1).into(),
            PieceMove::new(0, 1).into(),
            PieceRotate::new(1).into(),
            PieceRotate::new(2).into(),
            PieceRotate::new(3).into(),
            SoftDrop::new().into(),
        ];

        for command in commands {
            self.action(command);
            if !used.contains(&self.game.active) {
                used.insert(self.game.active);
                self.search(used);
            }
            self.undo();
        }
    }

    pub fn dfs(&mut self) -> HashSet<Placement> {
        let mut used = HashSet::new();
        self.search(&mut used);
        used
            .into_iter()
            .filter(
                |placement| self.game.board.piece_valid_placement(placement)
            ).collect()
    }

    pub fn build_pattern(&mut self, board: &Board) {
        for row in 0..board.height {
            println!("{}", self);
            self.build_row(board, row as i8);
        }
    }
    pub fn build_row(&mut self, board: &Board, row: i8) {
        loop {
            let placements = self.dfs();
            let mut ordered: Vec<ComparablePlacement> = placements
                .iter()
                .map(|&placement| ComparablePlacement {
                    placement,
                    board,
                    row
                }).collect();
            ordered.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let ComparablePlacement {placement, board, row} = ordered.pop().unwrap();
            if score(&placement, board, row) > 0 {
                self.game.active = ordered.pop().unwrap().placement;
                self.hard_drop();
                continue;
            }
            break
        }
    }

    pub fn undo(&mut self) {
        let mut command = self.stack.pop_front().unwrap();
        command.undo(&mut self.game);
    }

    pub fn move_left(&mut self) -> bool {
        self.action(PieceMove::new(0, -1).into())
    }

    pub fn move_right(&mut self) -> bool {
        self.action(PieceMove::new(0, 1).into())
    }

    pub fn soft_drop(&mut self) -> bool {
        self.action(SoftDrop::new().into())
    }

    pub fn rotate_cw(&mut self) -> bool {
        self.action(PieceRotate::new(1).into())
    }

    pub fn rotate_180(&mut self) -> bool {
        self.action(PieceRotate::new(2).into())
    }
    pub fn rotate_ccw(&mut self) -> bool {
        self.action(PieceRotate::new(3).into())
    }

    pub fn hard_drop(&mut self) -> bool {
        let sd = SoftDrop::new().into();
        let set = SetPiece::new().into();
        let clr = ClearLines::new().into();
        let nxt = NextPiece::new().into();
        let commands = Batch { commands: vec![sd, set, clr, nxt] };

        self.action(commands.into())
    }

    pub fn hold(&mut self) -> bool {
        self.action(Hold::new().into())
    }

    fn action(&mut self, command: Command) -> bool {
        self.stack.push_front(command);
        self.stack.get_mut(0).unwrap().execute(&mut self.game)
    }
}