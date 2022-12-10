use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

pub struct Queue {
    seed: usize,
    pieces: VecDeque<usize>,
    a: usize,
    m: usize,
}

impl Display for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.pieces)
    }
}

impl Queue {
    pub fn new(seed: usize) -> Self {
        Self {
            seed,
            pieces: VecDeque::new(),
            a: 16807,
            m: 2147483647,
        }
    }

    pub fn next(&mut self) -> usize {
        if self.pieces.len() < 10 {
            self.seven_bag()
        }

        self.pieces.pop_front().unwrap()
    }

    pub fn push(&mut self, piece: usize) {
        self.pieces.push_front(piece)
    }

    fn next_num(&mut self) -> f32 {
        self.seed = self.a * self.seed % self.m;

        let out = (self.seed - 1) as f32 / self.m as f32;
        out
    }

    fn seven_bag(&mut self) {
        let mut arr = [0, 1, 2, 3, 4, 5, 6];
        for i in (1..7).rev() {
            let r = (self.next_num() * (i as f32 + 1.0)) as usize;
            (arr[i], arr[r]) = (arr[r], arr[i])
        }
        self.pieces.extend(arr.iter());
    }
}
