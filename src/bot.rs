use std::fmt::{Display, Formatter};
use crate::game::*;
use crate::control::*;

pub struct Bot {
    game: Game,
    control: Control
}

impl Display for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.game)?;
        Ok(())
    }
}

impl Bot {
    pub fn immediate_moves(&mut self) {

    }
}