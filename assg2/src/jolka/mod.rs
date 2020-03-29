mod parser;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Full,
}

use std::fmt;

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "_"),
            Cell::Full => write!(f, "#")
        }
    }
}

type Board = Vec<Vec<Cell>>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Line {
    start_position: (usize, usize),
    axis: Axis,
    line_length: usize,
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = self.start_position;
        let end_position = match self.axis {
            Axis::Horizontal => (x + self.line_length, y),
            Axis::Vertical => (x, y + self.line_length)
        };
        write!(f, "({:?} -- {:?})", self.start_position, end_position)
    }
}

use crate::csp::Constraint;

#[derive(Clone)]
pub enum JolkaConstraint<'a> {
    Intersection {
        row: &'a Line,
        column: &'a Line,
        intersection_position: (usize, usize)
    },
    Length {
        line: &'a Line,
        line_length: usize,
    },
}

use std::collections::HashMap;

impl Constraint<'_, Jolka> for JolkaConstraint<'_> {
    fn is_satisfied(&self, env: &HashMap<Line, String>) -> bool {
        match self {
            Self::Intersection { row, column, intersection_position: (x, y) } => {
                match (&env.get(row), &env.get(column)) {
                    (Some(row_word), Some(column_word)) => {
                        let char_from_row_index = x - row.start_position.0;
                        let char_from_column_index = y - column.start_position.1;
                        row_word.chars().nth(char_from_row_index)
                            .map_or(false, |char_from_row: char| -> bool {
                                column_word.chars().nth(char_from_column_index)
                                    .map_or(false, |char_from_column: char| -> bool {
                                        char_from_row == char_from_column
                                    })
                            })
                    },
                    _ => true
                }
            },
            Self::Length { line, line_length } => {
                match &env.get(line) {
                    Some(word) => word.len() == *line_length,
                    None => true
                }
            }
        }
    }
}

impl fmt::Debug for JolkaConstraint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Intersection { row, column, intersection_position: (x, y) } => {
                let char_from_row_index = x - row.start_position.0;
                let char_from_column_index = y - column.start_position.1;
                write!(f, "({:?}[{}] == {:?}[{}])", row, char_from_row_index, column, char_from_column_index)
            },
            Self::Length { line, line_length } => {
                write!(f, "(len({:?}) == {})", line, line_length)
            }
        }
    }
}

use crate::csp::Solution;

pub struct JolkaSolution {
    board: Vec<Vec<Option<char>>>,
}

impl<'a> Solution<'a, Jolka> for JolkaSolution {
    fn construct(problem: &Jolka, assignments: &HashMap<Line, String>) -> JolkaSolution {
        let mut board: Vec<Vec<Option<char>>> = problem.board.iter().map(|row| {
            row.iter().map(|_| None).collect()
        }).collect();
        for (line, word) in assignments {
            for (i, ch) in word.chars().enumerate() {
                let (start_x, start_y) = line.start_position;
                let (x, y) = match line.axis {
                    Axis::Horizontal => (start_x + i, start_y),
                    Axis::Vertical => (start_x, start_y + i)
                };
                board[y][x] = Some(ch);
            }
        }
        JolkaSolution {
            board: board,
        }
    }
}

impl fmt::Display for JolkaSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for (y, row) in self.board.iter().enumerate() {
            for cell in row {
                match cell {
                    Some(ch) => result.push(*ch),
                    None => result.push('#')
                }
            }
            if y != self.board.len() - 1 {
                result.push('\n');
            }
        }
        write!(f, "{}", result)
    }
}

use crate::csp::CSP;

pub struct Jolka {
    board: Board,
    rows: Vec<Line>,
    columns: Vec<Line>,
    words: Vec<String>,
}

use std::path::PathBuf;

impl Jolka {
    pub fn load(board_path: &PathBuf, words_path: &PathBuf) -> Result<Jolka, parser::Error> {
        let board = parser::read_board(&board_path)?;
        let words = parser::read_words(&words_path)?;

        let rows = parser::parse_lines(&board, Axis::Horizontal);
        let columns = parser::parse_lines(&parser::transpose(&board), Axis::Vertical);

        Ok(Jolka {
            board: board,
            rows: rows,
            columns: columns,
            words: words,
        })
    }
}

impl<'a> CSP<'a> for Jolka {
    type Value = String;
    type Values = Words<'a>;
    type Variable = Line;
    type Constraint = JolkaConstraint<'a>;
    type Constraints = Vec<JolkaConstraint<'a>>;
    type Solution = JolkaSolution;

    fn constraints(&'a self) -> Vec<Self::Constraint> {
        let mut constraints = Vec::new();
        
        let mut rows_map: Vec<Vec<Option<&Line>>> = self.board.iter().map(|board_row| {
            board_row.iter().map(|_| None).collect()
        }).collect();
        for row in &self.rows {
            constraints.push(JolkaConstraint::Length {
                line: &row,
                line_length: row.line_length,
            });
            let (x, y) = row.start_position;
            for i in x..(x + row.line_length) {
                rows_map[y][i] = Some(&row);
            }
        }
        for column in &self.columns {
            constraints.push(JolkaConstraint::Length {
                line: &column,
                line_length: column.line_length,
            });
            let (x, y) = column.start_position;
            for i in y..(y + column.line_length) {
                if let Some(row_on_the_same_cell) = rows_map[i][x] {
                    constraints.push(JolkaConstraint::Intersection {
                        row: row_on_the_same_cell,
                        column: &column,
                        intersection_position: (x, i),
                    });
                }
            }
        }
        return constraints;
    }

    fn variables(&'a self) -> Vec<&Self::Variable> {
        let mut variables = Vec::new();
        for line in self.rows.iter().chain(self.columns.iter()) {
            variables.push(line);
        }
        return variables;
    }

    fn values(&'a self) -> Self::Values {
        Words {
            current: 0,
            source: &self.words,
        }
    }

    fn initial_assignments(&'a self) -> HashMap<Self::Variable, Self::Value> {
        HashMap::new()
    }
}

pub struct Words<'a> {
    source: &'a Vec<String>,
    current: usize,
}

impl Iterator for Words<'_> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let result = self.source.get(self.current).map(|word| word.clone());
        self.current += 1;
        return result;
    }
}
