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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct DiscDrop {
    pub column: usize,
    pub disc: Disc,
}

pub struct ValidDrops {
    board: Board,
    disc: Disc,
    current_column: usize,
}

impl Iterator for ValidDrops {
    type Item = DiscDrop;

    fn next(&mut self) -> Option<DiscDrop> {
        if self.current_column == self.board.columns.len() {
            return None;
        }
        while !self.board.is_valid_location(self.current_column) {
            self.current_column += 1;
            if self.current_column == self.board.columns.len() {
                return None;
            }
        }
        let result = Some(DiscDrop {
            column: self.current_column,
            disc: self.disc.clone(),
        });
        self.current_column += 1;
        return result;
    }
}

use crate::game::{Game, Node};

pub struct ConnectFour;

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
    
    fn is_terminal(&self, player: &Disc) -> bool {
        return self.check_for_win(player);
    }

    fn evaluate(&self, player: &Disc) -> i32 {
        use std::i32;
        use rand::Rng;

        let opponent = &player.next_player();

        if self.check_for_win(player) {
            return i32::MAX;
        }
        if self.check_for_win(opponent) {
            return i32::MIN;
        }
        
        let mut rng = rand::thread_rng();
        return rng.gen_range(0, 1000);
    }
}

impl Board {
    fn check_for_win(&self, player: &Disc) -> bool {
        // check columns
        for column in self.columns.iter() {
            for window in column.windows(4) {
                if window.iter().all(|disc| disc == player) {
                    return true;
                }
            }
        }
        // check rows
        for window in self.columns.windows(4) {
            let height = window.iter().map(|column| column.len()).min().unwrap();
            for y in 0..height {
                if window.iter().all(|column| column[y] == *player) {
                    return true;
                }
            }
        }
        // check '/' diagonals
        for window in self.columns.windows(4) {
            let height = window.iter().enumerate().map(|(x, column)| {
                column.len().checked_sub(x).unwrap_or(0)
            }).min().unwrap();
            for y in 0..height {
                if window.iter().enumerate().all(|(x, column)| column[y + x] == *player) {
                    return true;
                }
            }
        }
        // check '\' diagonals
        for window in self.columns.windows(4) {
            let height = window.iter().enumerate().map(|(x, column)| {
                column.len().checked_sub(4 - x - 1).unwrap_or(0)
            }).min().unwrap();
            for y in 0..height {
                if window.iter().enumerate().all(|(x, column)| column[y + 4  - x - 1] == *player) {
                    return true;
                }
            }
        }
        return false;
    }
}
