use wasm_bindgen::prelude::*;
use js_sys::Array;
use std::cell::RefCell;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Board {
    columns: Vec<Vec<Disc>>,
    bound: usize,
    four_in_a_row: RefCell<Option<((usize, usize), (i8, i8))>>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            columns: (0..width).map(|_| Vec::new()).collect(),
            bound: height,
            four_in_a_row: RefCell::new(None),
        }
    }

    #[wasm_bindgen(getter = columns)]
    pub fn get_columns(&self) -> Array {
        return self.columns.clone().into_iter().map(|column| -> Array {
            column.clone().into_iter().map(JsValue::from).collect()
        }).collect();
    }

    #[wasm_bindgen(getter = fourInARow)]
    pub fn get_four_in_a_row(&self) -> Option<Array> {
        self.four_in_a_row.borrow().map(|((x, y), (dx, dy))| {
            let position = Array::of2(&JsValue::from(x as u8), &JsValue::from(y as u8));
            let direction = Array::of2(&dx.into(), &dy.into());
            return Array::of2(&position, &direction);
        })
    }

    #[wasm_bindgen(getter = bound)]
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

    #[wasm_bindgen(js_name = checkForWin)]
    pub fn check_for_win(&self, player: Disc) -> bool {
        // check columns
        for (x, column) in self.columns.iter().enumerate() {
            for (y, window) in column.windows(4).enumerate() {
                if window.iter().all(|disc| *disc == player) {
                    *self.four_in_a_row.borrow_mut() = Some(((x, y), (0, 1)));
                    return true;
                }
            }
        }
        // check rows
        for (x, window) in self.columns.windows(4).enumerate() {
            let height = window.iter().map(|column| column.len()).min().unwrap();
            for y in 0..height {
                if window.iter().all(|column| column[y] == player) {
                    *self.four_in_a_row.borrow_mut() = Some(((x, y), (1, 0)));
                    return true;
                }
            }
        }
        // check '/' diagonals
        for (x, window) in self.columns.windows(4).enumerate() {
            let height = window.iter().enumerate().map(|(x, column)| {
                column.len().checked_sub(x).unwrap_or(0)
            }).min().unwrap();
            for y in 0..height {
                if window.iter().enumerate().all(|(x, column)| column[y + x] == player) {
                    *self.four_in_a_row.borrow_mut() = Some(((x, y), (1, 1)));
                    return true;
                }
            }
        }
        // check '\' diagonals
        for (x, window) in self.columns.windows(4).enumerate() {
            let height = window.iter().enumerate().map(|(x, column)| {
                column.len().checked_sub(4 - x - 1).unwrap_or(0)
            }).min().unwrap();
            for y in 0..height {
                if window.iter().enumerate().all(|(x, column)| column[y + 4 - x - 1] == player) {
                    *self.four_in_a_row.borrow_mut() = Some(((x, y + 3), (1, -1)));
                    return true;
                }
            }
        }
        return false;
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
#[derive(Copy, Clone)]
pub struct DiscDrop {
    pub column: usize,
    pub disc: Disc,
}

use std::fmt;

impl fmt::Debug for DiscDrop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.column.fmt(f)
    }
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
    
    fn is_terminal(&self) -> bool {
        // TODO: check for a draw
        if self.check_for_win(Disc::Yellow) {
            return true;
        } else if self.check_for_win(Disc::Red) {
            return true;
        }
        return false;
        // return self.check_for_win(Disc::Yellow) || self.check_for_win(Disc::Red);
    }

    fn evaluate(&self, maximizer: &Disc) -> i32 {
        use std::i32;

        let minimizer = &maximizer.next_player();

        let number_of_maximizers_discs: i32 = self.columns.iter().map(|column| -> i32 {
            column.iter().map(|disc| (disc == maximizer) as i32).sum()
        }).sum();

        if self.check_for_win(*maximizer) {
            // win in fewest number of discs placed
            return i32::MAX - number_of_maximizers_discs;
        }
        if self.check_for_win(*minimizer) {
            return i32::MIN + 100;
        }

        let mut score: i32 = 0;

        score += self.evaluate_windows(maximizer, |grouping| -> i32 {
            match grouping {
                (3, 1, 0) => 10,
                (2, 2, 0) => 6,
                (0, 1, 3) => -10,
                _ => 0
            }
        });

        return score;
    }
}

impl Board {
    fn evaluate_windows<F>(&self, player: &Disc, f: F) -> i32
        where F: Fn((usize, usize, usize)) -> i32
    {
        let count = |window: [&Option<Disc>; 4]| -> (usize, usize, usize) {
            let mut player_count = 0;
            let mut empty_count = 0;
            let mut opponent_count = 0;
            for disc in window.iter() {
                match disc {
                    Some(disc) => {
                        if disc == player {
                            player_count += 1;
                        } else {
                            opponent_count += 1;
                        }
                    },
                    None => empty_count += 1
                }
            }
            return (player_count, empty_count, opponent_count);
        };

        let option_board: Vec<Vec<Option<Disc>>> = self.columns.iter().map(|column| -> Vec<Option<Disc>> {
            (0..self.bound).map(|y| column.get(y).map(|disc| disc.clone())).collect()
        }).collect();
        let (width, height) = (self.columns.len(), self.bound);
        let mut score = 0;
        
        // evaluate columns
        for x in 0..width { // [0, 6]
            for y in 0..=height - 4 { // [0, 2]
                score += f(count([
                    &option_board[x][y],
                    &option_board[x][y + 1],
                    &option_board[x][y + 2],
                    &option_board[x][y + 3]
                ]));
            }
        }
        // evaluate rows
        for x in 0..=width - 4 { // [0, 3]
            for y in 0..height { // [0, 5]
                score += f(count([
                    &option_board[x][y],
                    &option_board[x + 1][y],
                    &option_board[x + 2][y],
                    &option_board[x + 3][y]
                ]));
            }
        }
        // evaluate '/' diagonals
        for x in 0..=width - 4 { // [0, 3]
            for y in 0..=height - 4 { // [0, 2]
                score += f(count([
                    &option_board[x][y],
                    &option_board[x + 1][y + 1],
                    &option_board[x + 2][y + 2],
                    &option_board[x + 3][y + 3]
                ]));
            }
        }
        // evaluate '\' diagonals
        for x in 0..=width - 4 { // [0, 3]
            for y in 0..=height - 4 { // [0, 2]
                score += f(count([
                    &option_board[x][y + 3],
                    &option_board[x + 1][y + 2],
                    &option_board[x + 2][y + 1],
                    &option_board[x + 3][y]
                ]));
            }
        }

        return score;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_for_win_in_a_row() {
        use Disc::{Red};

        let mut board = Board::new(7, 6);
        board.push(0, Red);
        board.push(1, Red);
        board.push(2, Red);
        board.push(3, Red);
        assert!(board.check_for_win(Red));
    }

    #[test]
    fn test_check_for_win_in_a_column() {
        use Disc::{Red};

        let mut board = Board::new(7, 6);
        board.push(0, Red);
        board.push(0, Red);
        board.push(0, Red);
        board.push(0, Red);
        assert!(board.check_for_win(Red));
    }

    #[test]
    fn test_check_for_win_in_a_diagonal() {
        use Disc::{Yellow, Red};

        let mut board = Board::new(7, 6);
        board.push(3, Yellow);
        board.push(3, Yellow);
        board.push(3, Yellow);
        board.push(3, Red);
        board.push(2, Yellow);
        board.push(2, Yellow);
        board.push(2, Red);
        board.push(1, Yellow);
        board.push(1, Red);
        board.push(0, Red);
        assert!(board.check_for_win(Red));
    }
}