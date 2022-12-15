#![allow(dead_code)]

use crate::board::Board;
use crate::control::*;
use crate::game::*;
use crate::piece::Placement;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::thread;
use std::sync::{Arc, Mutex};

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

#[derive(Clone)]
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

    fn add_rotations(&mut self, base: PlacementActions, used: &mut HashSet<PlacementActions>, before: Placement, check: bool) {
        for rotation in 0..4 {
            let mut rotation = PieceRotate::new(rotation);
            rotation.execute(&mut self.game);
            self.soft_drop();

            let mut base = base.clone();
            base.placement = self.game.active;
            base.push(rotation.into());
            base.push(SoftDrop::new().into());

            SoftDrop::new().execute(&mut self.game);
            let dropped = self.game.active;
            self.game.active = before;

            if !check || !duplicate_placement(used, &dropped) {
                used.insert(base);
            }
        }
    }

    fn trivial(&mut self, base: &mut PlacementActions, used: &mut HashSet<PlacementActions>) {
        let mut left = PieceMove::new(0, -1);
        let mut right = PieceMove::new(0, 1);
        let before = self.game.active;

        let mut left_base = base.clone();
        while left.execute(&mut self.game) {
            left_base.push(PieceMove::new(0, -1).into());
            self.add_rotations(left_base.clone(), used, self.game.active, false);
        }
        // left.undo(&mut self.game.active);
        self.game.active = before;

        let mut right_base = base.clone();
        while right.execute(&mut self.game) {
            right_base.push(PieceMove::new(0, 1).into());
            self.add_rotations(right_base.clone(), used, self.game.active, false);
        }
        // right.undo(&mut self.game.active);
        self.game.active = before;
    }

    fn non_trivial(&mut self, used: &mut HashSet<PlacementActions>) {
        let before = self.game.active;
        let mut unchecked = used.clone();

        for placement in used.clone() {
            self.game.active = placement.placement;
            self.add_rotations(placement, used, self.game.active, true);
        }
        self.game.active = before;
    }

    fn search_all(&mut self, base: &mut PlacementActions, used: &mut HashSet<PlacementActions>) {
        self.trivial(base, used);
        self.non_trivial(used);
    }

    fn search(&mut self) -> HashSet<PlacementActions> {
        let mut empty = PlacementActions::new();
        let mut hold = PlacementActions::new();
        hold.push(Hold::new().into());

        let mut other = self.clone();
        let h1 = thread::spawn(move || {
            let mut used = HashSet::new();
            other.search_all(&mut empty, &mut used);
            used
        });

        let mut used = HashSet::new();
        self.execute(Hold::new().into());
        self.search_all(&mut hold, &mut used);
        self.undo();

        h1
            .join()
            .unwrap()
            .into_iter()
            .chain(used)
            .into_iter()
            .filter(|placement| self.game.board.piece_valid_placement(&placement.placement))
            .collect::<HashSet<PlacementActions>>()
    }

    fn deep_search(&mut self, depth: usize, mut base: PlacementActions) -> HashSet<PlacementActions> {
        base.execute_last(&mut self.game);
        let search = self.search();

        if depth == 1 {
            base.undo_last(&mut self.game);

            return search
                .into_iter()
                .map(|placement| base.add_placement(placement))
                .collect();
        }

        let mut out = HashSet::new();
        for action in search {
            let mut base = base.clone();
            base.push(action.ret_push(HardDrop::new().into()).into());
            out.extend(self.deep_search(depth - 1, base));
        }

        base.undo_last(&mut self.game);
        out
    }

    pub fn look_ahead(&mut self, depth: usize) -> HashSet<PlacementActions> {
        self.deep_search(depth, PlacementActions::new())
    }

    pub fn undo(&mut self) {
        self.stack.pop_front().unwrap().undo(&mut self.game);
    }

    pub fn move_left(&mut self) -> bool {
        self.execute(PieceMove::new(0, -1).into())
    }

    pub fn move_right(&mut self) -> bool {
        self.execute(PieceMove::new(0, 1).into())
    }

    pub fn soft_drop(&mut self) -> bool {
        self.execute(SoftDrop::new().into())
    }

    pub fn rotate_cw(&mut self) -> bool {
        self.execute(PieceRotate::new(1).into())
    }

    pub fn rotate_180(&mut self) -> bool {
        self.execute(PieceRotate::new(2).into())
    }

    pub fn rotate_ccw(&mut self) -> bool {
        self.execute(PieceRotate::new(3).into())
    }

    pub fn hard_drop(&mut self) -> bool {
        self.execute(HardDrop::new().into())
    }

    pub fn hold(&mut self) -> bool {
        self.execute(Hold::new().into())
    }

    pub fn execute(&mut self, command: Command) -> bool {
        self.stack.push_front(command);
        self.stack.get_mut(0).unwrap().execute(&mut self.game)
    }
}
