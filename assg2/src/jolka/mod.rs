pub mod parser;

#[derive(PartialEq, Eq)]
pub enum State {
    On,
    Off,
}

use std::fmt;

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::On => write!(f, "_"),
            State::Off => write!(f, "#")
        }
    }
}

type Row = Vec<State>;
type Board = Vec<Row>;
