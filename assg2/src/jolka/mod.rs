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

use crate::csp::Variable;

pub struct Line {
    word: Option<String>,
    start_position: (usize, usize),
    line_length: usize,
}
    
impl Variable<String> for Line {}

use std::ops::{Deref, DerefMut};

impl Deref for Line {
    type Target = Option<String>;

    fn deref(&self) -> &Option<String> {
        &self.word
    }
}

impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Option<String> {
        &mut self.word
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.start_position.fmt(f)
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

impl Constraint for JolkaConstraint<'_> {
    fn is_satisfied(&self) -> bool {
        match self {
            Self::Intersection { row, column, intersection_position: (x, y) } => {
                match (&row.word, &column.word) {
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
                match &line.word {
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

        let rows = parser::parse_lines(&board, parser::Axis::Horizontal);
        let columns = parser::parse_lines(&parser::transpose(&board), parser::Axis::Vertical);

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

    fn variables(&'a mut self) -> Vec<&mut Self::Variable> {
        let mut variables = Vec::new();
        for line in self.rows.iter_mut().chain(self.columns.iter_mut()) {
            variables.push(&mut (*line));
        }
        return variables;
    }

    fn values(&'a self) -> Self::Values {
        Words {
            current: 0,
            source: &self.words,
        }
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
