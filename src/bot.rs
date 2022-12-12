#![allow(dead_code)]

use crate::board::Board;
use crate::control::*;
use crate::game::*;
use crate::piece::Placement;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};

pub fn score(piece: &Placement, board: &Board) -> i8 {
    let mut out = 0;
    for [y, x] in piece.rel_locations() {
        let y = y + piece.row;
        let x = x + piece.col;
        if board.get(y as usize, x as usize) {
            out -= 10;
        } else {
            out += 20;
        }
    }
    out
}

fn sort(board: Board) -> Box<dyn Fn(&PlacementActions, &PlacementActions) -> Ordering> {
    Box::new(move |a, b|
        score(&a.placement, &board).partial_cmp(&score(&b.placement, &board)).unwrap()
    )
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

    fn unfiltered_search(&mut self, base: &mut PlacementActions, used: &mut HashSet<PlacementActions>) {
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
                base.push(command);
                base.placement = self.game.active;
                used.insert(base.clone());
                self.unfiltered_search(base, used);
                base.pop();
            }
            self.undo();
            base.placement = self.game.active;
        }
    }

    fn search(&mut self, sort_predicate: Box<dyn Fn(&PlacementActions, &PlacementActions) -> Ordering>, n: usize) -> HashSet<PlacementActions> {
        let mut used = HashSet::new();
        let mut empty = PlacementActions::new();
        let mut hold = PlacementActions::new();
        hold.push(Hold::new().into());

        self.unfiltered_search(&mut empty, &mut used);

        self.action(hold.clone().into());
        self.unfiltered_search(&mut hold, &mut used);
        self.undo();

        used.into_iter()
            .filter(|placement| self.game.board.piece_valid_placement(&placement.placement))
            .sorted_by(sort_predicate)
            .take(n)
            .collect()
    }

    fn deep_search(&mut self, depth: usize, mut base: PlacementActions, n: usize, board: &Board) -> HashSet<PlacementActions> {
        base.execute_last(&mut self.game);
        let mut search = self.search(sort(board.clone()), n);
        search = search.into_iter().map(
            |placement| placement.ret_push_back(HardDrop::new().into())
        ).collect();

        if depth == 1 {
            base.undo_last(&mut self.game);
            return search
                .into_iter()
                .map(|action| base.ret_push_back(action.into()))
                .collect();
        }

        let mut out = HashSet::new();
        for action in search {
            let base = base.ret_push_back(action.into());
            out.extend(self.deep_search(depth - 1, base, n, board));
        }

        base.undo_last(&mut self.game);
        out
    }

    pub fn look_ahead(&mut self, depth: usize, n: usize, board: &Board) -> HashSet<PlacementActions> {
        self.deep_search(depth, PlacementActions::new(), n, board)
    }

    pub fn best_action(&mut self, depth: usize, n: usize, board: &Board) -> Command {
        self.look_ahead(depth, n, board)
            .into_iter()
            .sorted_by(sort(board.clone()))
            .next()
            .unwrap()
            .pop_front()
            .unwrap()
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
        self.stack.pop_front().unwrap().undo(&mut self.game)
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
