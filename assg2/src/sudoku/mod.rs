mod parser;

use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
pub enum Number {
    One = 1, Two = 2, Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7, Eight = 8, Nine = 9,
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self as u8).fmt(f)
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cell {
    position: (usize, usize),
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.position.fmt(f)
    }
}

use crate::csp::Constraint;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Unique<'a> {
    cell_a: &'a Cell,
    cell_b: &'a Cell,
}

use std::collections::HashMap;

impl Constraint<'_, Sudoku> for Unique<'_> {
    fn is_satisfied(&self, env: &HashMap<Cell, Number>) -> bool {
        match (env.get(self.cell_a), env.get(self.cell_b)) {
            (Some(a), Some(b)) => a != b,
            _ => true
        }
    }

    fn prune(&self, domains: &mut HashMap<Cell, HashSet<Number>>, variable: &Cell, value: &Number) -> usize {
        if variable == self.cell_a {
            let mut removed = 0;
            domains.get_mut(self.cell_b).map(|domain_b| {
                if domain_b.remove(value) {
                    removed += 1;
                }
            });
            return removed;
        } else if variable == self.cell_b {
            let mut removed = 0;
            domains.get_mut(self.cell_a).map(|domain_a| {
                if domain_a.remove(value) {
                    removed += 1;
                };
            });
            return removed;
        } else {
            return 0;
        }
    }
}

impl fmt::Debug for Unique<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?} != {:?})", self.cell_a, self.cell_b)
    }
}

use crate::csp::Solution;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct SudokuSolution {
    board: [[Number; 9]; 9],
}

impl<'a> Solution<'a, Sudoku> for SudokuSolution {
    fn construct(_: &Sudoku, assignments: &HashMap<Cell, Number>) -> SudokuSolution {
        let mut board: [[Number; 9]; 9] = [[Number::One; 9]; 9];
        for y in 0..9 {
            for x in 0..9 {
                board[y][x] = *assignments.get(&Cell {
                    position: (y, x),
                }).unwrap();
            }
        }
        SudokuSolution {
            board: board,
        }
    }
}

impl fmt::Debug for SudokuSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for y in 0..9 {
            for x in 0..9 {
                result.push_str(format!(" {:?}", self.board[y][x]).as_str());
                if x == 2 || x == 5 {
                    result.push_str(" |");
                }
            }
            if y != 8 {
                result.push_str(" \n")
            }
            if y == 2 || y == 5 {
                result.push_str("-------+-------+-------\n")
            }
        }
        write!(f, "{}", result)
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
        use log::info;

        let initial_board = parser::read_board(&path, &board_id)?;
        info!("Parsed the sudoku board file");

        Ok(Sudoku {
            board: parser::make_rows_of_cells(),
            initial_board: initial_board,
        })
    }
}

impl<'a> CSP<'a> for Sudoku {
    type Value = Number;
    type Variable = Cell;
    type Constraint = Unique<'a>;
    type Constraints = HashSet<Unique<'a>>;
    type Solution = SudokuSolution;

    fn constraints(&'a self) -> Self::Constraints {
        use itertools::Itertools;

        let mut constraints = HashSet::new();
        for group in parser::group_board_by_rows(&self.board).iter()
            .chain(parser::group_board_by_columns(&self.board).iter())
            .chain(parser::group_board_by_boxes(&self.board).iter())
        {
            for combination in group.iter().combinations(2) {
                constraints.insert(Unique {
                    cell_a: combination[0],
                    cell_b: combination[1],
                });
            }
        }
        return constraints;
    }

    fn domains(&'a self) -> HashMap<Cell, HashSet<Number>> {
        let mut domains = HashMap::new();
        for y in 0..9 {
            for x in 0..9 {
                domains.insert(self.board[y][x], match self.initial_board[y][x] {
                    Some(n) => [n].iter().cloned().collect(),
                    None => Numbers {
                        current: 0
                    }.collect()
                });
            }
        }
        return domains;
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
