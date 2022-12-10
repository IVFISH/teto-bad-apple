#![allow(dead_code)]

use crate::board::Board;
use crate::control::*;
use crate::game::*;
use crate::piece::Placement;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};

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
        score(&self.placement, &self.board, self.row).partial_cmp(&score(
            &other.placement,
            &other.board,
            other.row,
        ))
    }
}

fn score(piece: &Placement, board: &Board, row: i8) -> i8 {
    let mut out = 0;
    for [y, x] in piece.rel_locations() {
        let y = y + piece.row;
        let x = x + piece.col;
        if board.get(y as usize, x as usize) && row == y {
            out += 10;
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

    fn search(&mut self, base: &mut PlacementActions, used: &mut HashSet<PlacementActions>) {
        let commands: Vec<Command> = vec![
            PieceMove::new(0, -1).into(),
            PieceMove::new(0, 1).into(),
            PieceRotate::new(1).into(),
            PieceRotate::new(2).into(),
            PieceRotate::new(3).into(),
            SoftDrop::new().into(),
        ];

        for command in commands {
            if self.action(command.clone()) && !duplicate_placement(&used, &self.game.active) {
                base.batch.commands.push_back(command);
                base.placement = self.game.active;
                used.insert(base.clone());
                self.search(base, used);
                base.batch.commands.pop_back();
            }
            self.undo();
            base.placement = self.game.active;
        }
    }

    fn filtered_search(&mut self, base: &mut PlacementActions) -> HashSet<PlacementActions> {
        // maybe add hold sometime
        let mut used = HashSet::new();
        self.action(base.clone().into());
        self.search(base, &mut used);
        self.undo();

        used.into_iter()
            .filter(|placement| self.game.board.piece_valid_placement(&placement.placement))
            .collect()
    }

    fn deep_search(&mut self, depth: usize, base: &mut PlacementActions) -> HashSet<PlacementActions> {
        let first = self.filtered_search(base);

        if depth == 1 {
            return first;
        }

        let mut out = HashSet::new();

        for mut action in first {
            action.batch.commands.push_back(HardDrop::new().into());
            out.extend(self.deep_search(depth - 1, &mut action));
        }
        out
    }

    pub fn look_ahead(&mut self, depth: usize) -> HashSet<PlacementActions> {
        self.deep_search(depth, &mut PlacementActions { batch: Batch::new(), placement: self.game.active })
    }

    // pub fn build_pattern(&mut self, board: &Board) {
    //     for row in 0..board.height {
    //         println!("{}", self);
    //         self.build_row(board, row as i8);
    //     }
    // }

    // pub fn build_row(&mut self, board: &Board, row: i8) {
    //     loop {
    //         let placements = self.filtered_search();
    //         let mut ordered: Vec<ComparablePlacement> = placements
    //             .iter()
    //             .map(|placement| ComparablePlacement {
    //                 placement: placement.placement,
    //                 board,
    //                 row,
    //             })
    //             .collect();
    //         ordered.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //         let ComparablePlacement {
    //             placement,
    //             board,
    //             row,
    //         } = ordered.pop().unwrap();
    //         if score(&placement, board, row) > 0 {
    //             self.game.active = ordered.pop().unwrap().placement;
    //             self.hard_drop();
    //             continue;
    //         }
    //         break;
    //     }
    // }

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
        self.action(HardDrop::new().into())
    }

    pub fn hold(&mut self) -> bool {
        self.action(Hold::new().into())
    }

    pub fn action(&mut self, command: Command) -> bool {
        self.stack.push_front(command);
        self.stack.get_mut(0).unwrap().execute(&mut self.game)
    }
}
