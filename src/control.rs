#![allow(dead_code)]

use crate::game::*;
use crate::piece::{Placement, Point};
use enum_dispatch::enum_dispatch;
use std::collections::{HashSet, VecDeque};

#[enum_dispatch]
#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Command {
    PieceRotate,
    PieceMove,
    SoftDrop,
    SetPiece,
    NextPiece,
    Hold,
    ClearLines,
    Batch,
}

#[enum_dispatch(Command)]
pub trait Executable {
    fn execute(&mut self, game: &mut Game) -> bool;
    fn undo(&mut self, game: &mut Game);
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct PieceMove {
    moved: bool,
    dy: i8,
    dx: i8,
}

impl PieceMove {
    pub fn new(dy: i8, dx: i8) -> Self {
        Self {
            moved: false,
            dy,
            dx,
        }
    }
}

impl Executable for PieceMove {
    fn execute(&mut self, game: &mut Game) -> bool {
        game.active.shift(self.dy, self.dx);
        self.moved = game.board.piece_valid_location(&game.active);
        if !self.moved {
            game.active.shift(-self.dy, -self.dx);
        };
        self.moved
    }

    fn undo(&mut self, game: &mut Game) {
        if self.moved {
            game.active.shift(-self.dy, -self.dx);
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct PieceRotate {
    direction: usize,
    before: Placement,
}

impl PieceRotate {
    pub fn new(direction: usize) -> Self {
        Self {
            direction,
            before: Placement::default(),
        }
    }
}

impl Executable for PieceRotate {
    fn execute(&mut self, game: &mut Game) -> bool {
        self.before = game.active;
        game.active.rotate(self.direction);

        for [y, x] in game.active.get_offsets(self.direction) {
            let mut command = PieceMove::new(-y, -x);
            // println!("{} {}", x, y);
            if command.execute(game) {
                return true;
            }
        }

        game.active.rotate(4 - self.direction);
        false
    }

    fn undo(&mut self, game: &mut Game) {
        game.active = self.before;
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct SoftDrop {
    distance: i8,
}

impl SoftDrop {
    pub fn new() -> Self {
        Self { distance: 0 }
    }
}

impl Executable for SoftDrop {
    fn execute(&mut self, game: &mut Game) -> bool {
        let mut down = PieceMove::new(-1, 0);
        while down.execute(game) {
            self.distance += 1;
        }
        true
    }

    fn undo(&mut self, game: &mut Game) {
        game.active.shift(self.distance, 0);
    }
}

#[derive(Default, Clone, Hash, Eq, PartialEq)]
pub struct SetPiece {
    locations: [Point; 4],
    row: i8,
    col: i8,
}

impl SetPiece {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Executable for SetPiece {
    fn execute(&mut self, game: &mut Game) -> bool {
        (self.locations, self.row, self.col) = (
            game.active.rel_locations(),
            game.active.row,
            game.active.col,
        );
        for [r, c] in self.locations {
            game.board
                .add((r + self.row) as usize, (c + self.col) as usize);
        }
        true
    }

    fn undo(&mut self, game: &mut Game) {
        for [r, c] in self.locations {
            game.board
                .remove((r + self.row) as usize, (c + self.col) as usize);
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct NextPiece {
    cur_piece: Placement,
    next_piece: usize,
}

impl Executable for NextPiece {
    fn execute(&mut self, game: &mut Game) -> bool {
        self.cur_piece = game.active;
        self.next_piece = game.queue.next();
        game.active = game.new_piece(self.next_piece);
        true
    }

    fn undo(&mut self, game: &mut Game) {
        game.queue.push(self.next_piece);
        game.active = self.cur_piece;
    }
}

impl NextPiece {
    pub fn new() -> Self {
        Self {
            cur_piece: Placement::default(),
            next_piece: 8,
        }
    }
}

#[derive(Default, Clone, Hash, Eq, PartialEq)]
pub struct Hold {
    first: bool,
    before: usize,
    after: usize,
}

impl Hold {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Executable for Hold {
    fn execute(&mut self, game: &mut Game) -> bool {
        self.before = game.active.piece_type;
        self.first = game.hold.is_none();
        if self.first {
            NextPiece::new().execute(game);
        } else {
            game.active = game.new_piece(game.hold.unwrap());
        }

        self.after = game.active.piece_type;
        game.hold = Some(self.before);
        true
    }

    fn undo(&mut self, game: &mut Game) {
        if self.first {
            game.hold = None
        } else {
            game.hold = Some(self.before);
        }
        game.queue.push(self.after);
        game.active = game.new_piece(self.before);
    }
}

#[derive(Default, Clone, Hash, Eq, PartialEq)]
pub struct ClearLines {
    line_indices: Vec<(usize, Vec<bool>)>,
}

impl ClearLines {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Executable for ClearLines {
    fn execute(&mut self, game: &mut Game) -> bool {
        for row in 0..game.board.height {
            if let Some(line) = game.board.line_clear(row) {
                self.line_indices.push((row, line));
            }
        }
        true
    }

    fn undo(&mut self, game: &mut Game) {
        self.line_indices.reverse();
        while let Some((index, vec)) = self.line_indices.pop() {
            game.board.arr.insert(index, vec);
        }
    }
}

#[derive(Default, Clone, Hash, Eq, PartialEq)]
pub struct Batch {
    pub commands: Vec<Command>,
}

impl Batch {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Executable for Batch {
    fn execute(&mut self, game: &mut Game) -> bool {
        for command in self.commands.iter_mut() {
            // should have some behavior for when this doesn't work (aka undo everything)
            // but nah what can go wrong
            command.execute(game);
        }
        true
    }

    fn undo(&mut self, game: &mut Game) {
        for command in self.commands.iter_mut().rev() {
            command.undo(game);
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Default)]
pub struct PlacementActions {
    pub batch: Batch,
    pub placement: Placement
}

impl PlacementActions {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn duplicate_placement(used: &HashSet<PlacementActions>, piece: &Placement) -> bool {
    used.iter().any(|PlacementActions {batch: _, placement}| placement == piece)
}