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

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
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
pub struct Intersection<'a> {
    row: &'a Line,
    column: &'a Line,
    intersection_position: (usize, usize)
}

use std::collections::{HashMap, HashSet};

impl Constraint<'_, Jolka> for Intersection<'_> {
    fn is_satisfied(&self, env: &HashMap<Line, String>) -> bool {
        match (&env.get(self.row), &env.get(self.column)) {
            (Some(row_word), Some(column_word)) => {
                let (x, y) = self.intersection_position;
                let char_from_row_index = x - self.row.start_position.0;
                let char_from_column_index = y - self.column.start_position.1;
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
    }

    fn prune(&self, domains: &mut HashMap<Line, HashSet<String>>, variable: &Line, value: &String) -> usize {
        let (start_x, _) = self.row.start_position;
        let (_, start_y) = self.column.start_position;
        let (x, y) = self.intersection_position;
        if variable == self.row {
            let mut removed = 0;
            let character = value.chars().nth(x - start_x).unwrap();
            domains.get_mut(self.column).map(|domain| {
                for word in domain.clone() {
                    if let Some(other_character) = word.chars().nth(y - start_y) {
                        if other_character != character {
                            if domain.remove(&word) {
                                removed += 1;
                            }
                        }
                    }
                }
            });
            return removed;
        } else if variable == self.column {
            let mut removed = 0;
            let character = value.chars().nth(y - start_y).unwrap();
            domains.get_mut(self.row).map(|domain| {
                for word in domain.clone() {
                    if let Some(other_character) = word.chars().nth(x - start_x) {
                        if other_character != character {
                            if domain.remove(&word) {
                                removed += 1;
                            }
                        }
                    }
                }
            });
            return removed;
        } else {
            return 0;
        }
    }
}

impl fmt::Debug for Intersection<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char_from_row_index = self.intersection_position.0 - self.row.start_position.0;
        let char_from_column_index = self.intersection_position.1 - self.column.start_position.1;
        write!(f, "({:?}[{}] == {:?}[{}])", self.row, char_from_row_index, self.column, char_from_column_index)
    }
}

use crate::csp::Solution;

#[derive(Eq, PartialEq, Hash, Clone)]
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

impl fmt::Debug for JolkaSolution {
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
        use log::info;

        let board = parser::read_board(&board_path)?;
        info!("Parsed the jolka board file");
        let words = parser::read_words(&words_path)?;
        info!("Parsed the jolka words file");

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
    type Variable = Line;
    type Constraint = Intersection<'a>;
    type Constraints = Vec<Intersection<'a>>;
    type Solution = JolkaSolution;

    fn constraints(&'a self) -> Vec<Self::Constraint> {
        let mut constraints = Vec::new();

        let mut rows_map: Vec<Vec<Option<&Line>>> = self.board.iter().map(|board_row| {
            board_row.iter().map(|_| None).collect()
        }).collect();
        for row in &self.rows {
            let (x, y) = row.start_position;
            for i in x..(x + row.line_length) {
                rows_map[y][i] = Some(&row);
            }
        }
        for column in &self.columns {
            let (x, y) = column.start_position;
            for i in y..(y + column.line_length) {
                if let Some(row_on_the_same_cell) = rows_map[i][x] {
                    constraints.push(Intersection {
                        row: row_on_the_same_cell,
                        column: &column,
                        intersection_position: (x, i),
                    });
                }
            }
        }
        return constraints;
    }

    fn domains(&self) -> HashMap<Line, HashSet<String>> {
        let mut domains = HashMap::new();
        for line in self.rows.iter().chain(self.columns.iter()) {
            let mut domain = HashSet::new();
            for word in self.words.iter().cloned() {
                if word.len() == line.line_length {
                    domain.insert(word);
                }
            }
            domains.insert(line.to_owned(), domain);
        }
        return domains;
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
