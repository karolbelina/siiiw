mod parser;

use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Number {
    One = 1, Two = 2, Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7, Eight = 8, Nine = 9,
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self as u8).fmt(f)
    }
}

use crate::csp::Variable;

#[derive(Copy, Clone, Default)]
pub struct Cell {
    value: Option<Number>,
    position: (usize, usize)
}
    
impl Variable<Number> for Cell {}

use std::ops::{Deref, DerefMut};

impl Deref for Cell {
    type Target = Option<Number>;

    fn deref(&self) -> &Option<Number> {
        &self.value
    }
}

impl DerefMut for Cell {
    fn deref_mut(&mut self) -> &mut Option<Number> {
        &mut self.value
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.position.fmt(f)
    }
}

use crate::csp::Constraint;

#[derive(Clone)]
pub enum SudokuConstraint<'a> {
    Unique {
        cell_a: &'a Cell,
        cell_b: &'a Cell,
    },
    Fixed {
        cell: &'a Cell,
        value: Number,
    },
}

impl Constraint for SudokuConstraint<'_> {
    fn is_satisfied(&self) -> bool {
        match self {
            Self::Unique { cell_a, cell_b } => {
                match (cell_a.value, cell_b.value) {
                    (Some(a), Some(b)) => a != b,
                    _ => true
                }
            },
            Self::Fixed { cell, value } => {
                match cell.value {
                    Some(n) => n == *value,
                    None => true
                }
            }
        }
    }
}

impl PartialEq for SudokuConstraint<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unique { cell_a, cell_b }, Self::Unique { cell_a: other_a, cell_b: other_b }) => {
                *cell_a as *const Cell == *other_a as *const Cell && *cell_b as *const Cell == *other_b as *const Cell
            },
            (Self::Fixed { cell, .. }, Self::Fixed { cell: other, .. }) => {
                *cell as *const Cell == *other as *const Cell
            },
            _ => false
        }
    }
}

impl Eq for SudokuConstraint<'_> {}

use std::hash::{Hash, Hasher};

impl Hash for SudokuConstraint<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Unique { cell_a, cell_b } => {
                cell_a.hash(state);
                cell_b.hash(state);
            },
            Self::Fixed { cell, .. } => {
                cell.hash(state)
            }
        }
    }
}

impl fmt::Debug for SudokuConstraint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unique { cell_a, cell_b } => {
                write!(f, "({:?} != {:?})", cell_a, cell_b)
            },
            Self::Fixed { cell, value } => {
                write!(f, "({:?} == {:?})", cell, value)
            }
        }
    }
}

use crate::csp::CSP;
use std::collections::HashSet;

pub struct Sudoku {
    board: [[Cell; 9]; 9],
    initial_board: [[Option<Number>; 9]; 9],
}

use std::path::PathBuf;

impl Sudoku {
    pub fn load(path: &PathBuf, board_id: &String) -> Result<Sudoku, parser::Error> {
        let initial_board = parser::read_board(&path, &board_id)?;

        Ok(Sudoku {
            board: parser::make_rows_of_cells(),
            initial_board: initial_board,
        })
    }
}

impl<'a> CSP<'a> for Sudoku {
    type Value = Number;
    type Values = Numbers;
    type Variable = Cell;
    type Constraint = SudokuConstraint<'a>;
    type Constraints = HashSet<SudokuConstraint<'a>>;

    fn constraints(&'a self) -> Self::Constraints {
        use itertools::Itertools;

        let mut constraints = HashSet::new();
        for y in 0..9 {
            for x in 0..9 {
                if let Some(value) = self.initial_board[y][x] {
                    constraints.insert(SudokuConstraint::Fixed {
                        cell: &self.board[y][x],
                        value: value,
                    });
                }
            }
        }
        for group in parser::group_board_by_rows(&self.board).iter()
            .chain(parser::group_board_by_columns(&self.board).iter())
            .chain(parser::group_board_by_boxes(&self.board).iter())
        {
            for combination in group.iter().combinations(2) {
                constraints.insert(SudokuConstraint::Unique {
                    cell_a: combination[0],
                    cell_b: combination[1],
                });
            }
        }
        return constraints;
    }

    fn variables(&'a mut self) -> Vec<&mut Self::Variable> {
        let mut variables = Vec::new();
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                variables.push(cell);
            }
        }
        return variables;
    }

    fn values(&'a self) -> Self::Values {
        Numbers {
            current: 0,
        }
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

// pub struct Fields<'a> {
//     current: usize,
//     board: &'a mut Board,
// }

// impl<'a> Iterator for Fields<'a> {
//     type Item = &'a mut Field;

//     fn next(&mut self) -> Option<&'a mut Field> {
//         if self.current < 81 {
//             let x = self.current % 9;
//             let y = self.current / 9;
//             self.current += 1;
//             let value = self.board.get_mut(x).and_then(|col| col.get_mut(y));
//             value.map(|inner| unsafe { &mut *(inner as *mut _) })
//         } else {
//             None
//         }
//     }
// }

// use crate::csp::{CSP, Variables};

// impl<'a> Variables<'a> for [[Field; 9]; 9] {
//     type Item = Field;
//     type Iter = Fields<'a>;

//     fn variables(&'a mut self) -> Fields<'a> {
//         Fields {
//             current: 0,
//             board: &mut self,
//         }
//     }
// }

// impl<'a> CSP<'a> for Sudoku {
//     type Value = Number;
//     type Variable = Field;
//     type Values = Numbers;
//     type State = Board;

//     fn get_initial_state(&self) -> Board {
//         return self.board;
//     }

//     fn values() -> Numbers {
//         Numbers {
//             current: 0,
//         }
//     }

//     fn is_satisfied(&self, state: &Board) -> bool {
//         unimplemented!()
//     }
// }
