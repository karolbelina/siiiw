mod parser;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Number {
    One = 1, Two = 2, Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7, Eight = 8, Nine = 9,
}

use std::fmt;

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self as u8).fmt(f)
    }
}

#[derive(Copy, Clone)]
pub struct Field(Option<Number>);

use std::ops::{Deref, DerefMut};

impl Deref for Field {
    type Target = Option<Number>;

    fn deref(&self) -> &Option<Number> {
        &self.0
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut Option<Number> {
        &mut self.0
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

type Board = [[Field; 9]; 9];

pub struct Sudoku {
    board: Board,
}

use std::path::PathBuf;

impl Sudoku {
    pub fn load(path: &PathBuf, board_id: &String) -> Result<Sudoku, parser::Error> {
        let board_state = parser::read_board(&path, &board_id)?;
        let mut board = [[Field(None); 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = Field(board_state[i][j]);
            }
        }

        Ok(Sudoku {
            board: board,
        })
    }
}

pub struct Numbers {
    current: u8
}

impl Iterator for Numbers {
    type Item = Number;

    fn next(&mut self) -> Option<Number> {
        self.current += 1;
        match self.current {
            1 => Some(Number::One),
            2 => Some(Number::Two),
            3 => Some(Number::Three),
            4 => Some(Number::Four),
            5 => Some(Number::Five),
            6 => Some(Number::Six),
            7 => Some(Number::Seven),
            8 => Some(Number::Eight),
            9 => Some(Number::Nine),
            _ => None
        }
    }
}

pub struct Fields<'a> {
    current: usize,
    board: &'a mut Board,
}

impl<'a> Iterator for Fields<'a> {
    type Item = &'a mut Field;

    fn next(&mut self) -> Option<&'a mut Field> {
        if self.current < 81 {
            let x = self.current % 9;
            let y = self.current / 9;
            self.current += 1;
            let value = self.board.get_mut(x).and_then(|col| col.get_mut(y));
            value.map(|inner| unsafe { &mut *(inner as *mut _) })
        } else {
            None
        }
    }
}

use crate::csp::CSP;

impl<'a> CSP<'a> for Sudoku {
    type Value = Number;
    type Variable = Field;
    type Values = Numbers;
    type Variables = Fields<'a>;

    fn values() -> Numbers {
        Numbers {
            current: 0,
        }
    }

    fn variables<'b>(&'b mut self) -> Fields<'b> {
        Fields {
            current: 0,
            board: &mut self.board,
        }
    }
}
