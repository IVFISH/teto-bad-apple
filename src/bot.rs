#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use crate::board::Board;
use crate::game::*;
use crate::control::*;
use crate::piece::Placement;

pub struct ComparablePlacement<'a> {
    pub placement: Placement,
    pub board: &'a Board,
}


impl PartialEq<Self> for ComparablePlacement<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.placement.eq(&other.placement)
    }
}

impl PartialOrd for ComparablePlacement<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        println!("{}{}", score(&self.placement, &self.board), &score(&other.placement, &other.board));
        score(&self.placement, &self.board)
            .partial_cmp(&score(&other.placement, &other.board))
    }
}

fn score(piece: &Placement, board: &Board) -> i8 {
    let mut out = 0;
    for [y, x] in piece.rel_locations() {
        let y = (y + piece.row) as usize;
        let x = (x + piece.col) as usize;
        if board.get(y, x) {
            out -= 1;
        } else {
            out += 2;
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
        loop {
            let placements = self.dfs();
            println!("{}", placements.len());
            let mut ordered: Vec<ComparablePlacement> = placements
                .iter()
                .map(|&placement| ComparablePlacement {
                    placement,
                    board,
                }).collect();
            println!("{}", ordered.len());
            ordered.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let ComparablePlacement {placement, board} = ordered.pop().unwrap();
            if score(&placement, board) < 0 {
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
        let nxt = NextPiece::new().into();
        let commands = Batch { commands: vec![sd, set, nxt] };

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