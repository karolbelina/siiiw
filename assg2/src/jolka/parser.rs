use snafu::{ResultExt, Snafu};
use super::{Cell, Board};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not load the board file: {}", source))]
    LoadBoard { source: std::io::Error },
    #[snafu(display("Could not load the words file: {}", source))]
    LoadWords { source: std::io::Error },
    #[snafu(display("Invalid character {:?} on line {} at position {}", character, line, position))]
    InvalidCharacter { character: char, line: usize, position: usize },
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn parse_board(source: &str) -> Result<Board> {
    return source.split_whitespace().enumerate().map(|(i, line)| -> Result<Vec<Cell>> {
        line.chars().enumerate().map(|(j, c)| -> Result<Cell> {
            match c {
                '_' => Ok(Cell::Empty),
                '#' => Ok(Cell::Full),
                 _  => Err(Error::InvalidCharacter { character: c, line: i, position: j })
            }
        }).collect()
    }).collect::<Result<Board>>();
}

fn parse_words(source: &str) -> Vec<String> {
    source.split_whitespace().map(String::from).collect()
}

use std::path::PathBuf;

pub fn read_board(path: &PathBuf) -> Result<Board> {
    use std::fs;

    let contents = fs::read_to_string(path).context(LoadBoard)?;
    return parse_board(&contents);
}

pub fn read_words(path: &PathBuf) -> Result<Vec<String>> {
    use std::fs;

    let contents = fs::read_to_string(path).context(LoadBoard)?;
    return Ok(parse_words(&contents));
}

pub fn transpose(rows: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut columns = Vec::new();
    let column_number = rows[0].len();
    for x in 0..column_number {
        let mut column = Vec::new();
        for row in rows {
            column.push(row[x]);
        }
        columns.push(column);
    }
    return columns;
}

use super::Line;

fn get_cells_occupied(line: &Vec<Cell>) -> Vec<Vec<usize>> {
    let mut occupied: Vec<Vec<usize>> = Vec::new();
    let mut current_start: usize = 0;
    let mut empties_in_a_row: usize = 0;
    for (i, board_cell) in line.iter().enumerate() {
        match board_cell {
            Cell::Empty => {
                if empties_in_a_row == 0 {
                    current_start = i;
                }
                empties_in_a_row += 1;
            },
            Cell::Full => {
                if empties_in_a_row > 1 {
                    occupied.push((current_start..i).collect());
                }
                empties_in_a_row = 0;
            }
        }
    }
    if empties_in_a_row > 1 {
        occupied.push((current_start..line.len()).collect());
    }
    return occupied;
}

use super::Axis;

pub fn parse_lines(lines: &Vec<Vec<Cell>>, axis: Axis) -> Vec<Line> {
    let mut rows = Vec::new();
    for (i, board_row) in lines.iter().enumerate() {
        let columns_occupied = get_cells_occupied(&board_row);
        for line in columns_occupied {
            let row = Line {
                start_position: match axis {
                    Axis::Horizontal => (line[0], i),
                    Axis::Vertical => (i, line[0])
                },
                axis: axis,
                line_length: line.len(),
            };
            rows.push(row);
        }
    }
    return rows;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_board() {
        use super::{parse_board, Cell::Empty, Cell::Full};

        assert_eq!(
            parse_board("___##__#\n#____#_#\n__#___##").unwrap(),
            vec!(
                vec!(Empty, Empty, Empty, Full, Full, Empty, Empty, Full),
                vec!(Full, Empty, Empty, Empty, Empty, Full, Empty, Full),
                vec!(Empty, Empty, Full, Empty, Empty, Empty, Full, Full),
            )
        );
    }

    #[test]
    fn test_parse_board_error() {
        use super::parse_board;

        assert!(parse_board("___#\n##;_").is_err());
    }

    #[test]
    fn test_parse_words() {
        use super::parse_words;

        assert_eq!(
            parse_words("ABC\nTEST\nIJK"),
            vec!("ABC", "TEST", "IJK")
        )
    }

    #[test]
    fn test_transpose() {
        use super::{transpose, Cell::Empty, Cell::Full};

        let rows = vec!(
            vec!(Empty, Empty, Empty, Full),
            vec!(Full, Empty, Empty, Full),
            vec!(Empty, Empty, Full, Full)
        );

        assert_eq!(
            transpose(&rows),
            vec!(
                vec!(Empty, Full, Empty),
                vec!(Empty, Empty, Empty),
                vec!(Empty, Empty, Full),
                vec!(Full, Full, Full)
            )
        )
    }
}
