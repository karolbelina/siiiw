use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Board {
    columns: Vec<Vec<Disc>>,
    bound: usize,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            columns: (0..width).map(|_| Vec::new()).collect(),
            bound: height,
        }
    }

    #[wasm_bindgen(js_name = getColumns)]
    pub fn get_columns(&self) -> Array {
        return self.columns.clone().into_iter().map(|column| -> Array {
            column.clone().into_iter().map(JsValue::from).collect()
        }).collect();
    }

    #[wasm_bindgen(js_name = getBound)]
    pub fn get_bound(&self) -> usize {
        return self.bound;
    }

    pub fn push(&mut self, index: usize, disc: Disc) {
        self.columns[index].push(disc)
    }

    #[wasm_bindgen(js_name = isValidLocation)]
    pub fn is_valid_location(&self, index: usize) -> bool {
        self.columns[index].len() < self.bound
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Disc {
    Yellow = 0,
    Red = 1,
}

impl From<Disc> for JsValue {
    #[inline]
    fn from(disc: Disc) -> JsValue {
        JsValue::from(disc as u8)
    }
}

use crate::game::NextPlayer;

impl NextPlayer for Disc {
    fn next_player(&self) -> Disc {
        match self {
            Disc::Yellow => Disc::Red,
            Disc::Red => Disc::Yellow
        }
    }
}

impl Default for Disc {
    fn default() -> Disc {
        Disc::Yellow
    }
}

#[derive(Copy, Clone)]
pub struct DiscDrop {
    column: usize,
    disc: Disc,
}

pub struct ValidDrops {
    board: Board,
    disc: Disc,
    current_column: usize,
}

impl Iterator for ValidDrops {
    type Item = DiscDrop;

    fn next(&mut self) -> Option<DiscDrop> {
        while !self.board.is_valid_location(self.current_column) {
            self.current_column += 1;
            if self.current_column == self.board.columns.len() {
                return None;
            }
        }
        Some(DiscDrop {
            column: self.current_column,
            disc: self.disc.clone(),
        })
    }
}

use crate::game::{Game, Node};

struct ConnectFour;

impl Game for ConnectFour {
    type State = Board;
    type Player = Disc;
}

impl Node<ConnectFour> for Board {
    type Decision = DiscDrop;
    type Decisions = ValidDrops;

    fn decisions(&self, player: Disc) -> ValidDrops {
        ValidDrops {
            board: self.clone(),
            disc: player,
            current_column: 0,
        }
    }

    fn make_decision(&self, decision: DiscDrop) -> Board {
        let mut child = self.clone();
        let DiscDrop { column, disc } = decision;
        child.push(column, disc);
        return child;
    }

    fn evaluate(&self, player: &Disc) -> i32 {
        unimplemented!()
    }
}
