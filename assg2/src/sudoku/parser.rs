use snafu::{ResultExt, Snafu, ensure};

use super::Number;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not load the board: {}", source))]
    LoadBoard { source: csv::Error },
    #[snafu(display("No board with id {:?} found", board_id))]
    InvalidBoardId { board_id: String },
    #[snafu(display("Invalid character {:?} at position {}", character, position))]
    InvalidCharacter { character: char, position: usize },
    #[snafu(display("Not enough fields provided"))]
    NotEnoughFields,
    #[snafu(display("Too many fields provided"))]
    TooManyFields,
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn get_board_string(path: &PathBuf, board_id: &String) -> Result<String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .flexible(true)
        .from_path(path)
        .context(LoadBoard)?;
    for result in reader.records() {
        let record = result.context(LoadBoard)?;
        if record[0] == *board_id {
            return Ok(record[2].to_owned());
        }
    }
    return Err(Error::InvalidBoardId { board_id: board_id.clone() });
}

fn parse_board(source: &str) -> Result<[[Option<Number>; 9]; 9]> {
    let numbers = source.chars().enumerate().map(|(i, c)| {
        match c {
            '1' => Ok(Some(Number::One)),
            '2' => Ok(Some(Number::Two)),
            '3' => Ok(Some(Number::Three)),
            '4' => Ok(Some(Number::Four)),
            '5' => Ok(Some(Number::Five)),
            '6' => Ok(Some(Number::Six)),
            '7' => Ok(Some(Number::Seven)),
            '8' => Ok(Some(Number::Eight)),
            '9' => Ok(Some(Number::Nine)),
            '.' => Ok(None),
             _  => Err(Error::InvalidCharacter { character: c, position: i })
        }
    }).collect::<Result<Vec<Option<Number>>, Error>>()?;
    let mut iter = numbers.iter();
    let mut board = [[None; 9]; 9];
    for i in 0..81 {
        match iter.next() {
            Some(v) => board[i % 9][i / 9] = *v,
            None => return Err(Error::NotEnoughFields)
        }
    }
    ensure!(iter.next().is_none(), TooManyFields);
    return Ok(board);
}

pub fn read_board(path: &PathBuf, board_id: &String) -> Result<[[Option<Number>; 9]; 9]> {
    let board_string = get_board_string(path, &board_id)?;
    return parse_board(&board_string);
}

use super::Cell;

pub fn make_rows_of_cells() -> [[Cell; 9]; 9] {
    let mut rows: [[Cell; 9]; 9] = [[Cell::default(); 9]; 9];
    for y in 0..9 {
        for x in 0..9 {
            rows[y][x].position = (x, y)
        }
    }
    return rows;
}

pub fn group_board_by_rows<T>(board: &[[T; 9]; 9]) -> [[&T; 9]; 9] {
    let mut rows: [[&T; 9]; 9] = [[&board[0][0]; 9]; 9];
    for y in 0..9 {
        for x in 0..9 {
            rows[y][x] = &board[y][x]
        }
    }
    return rows;
}

pub fn group_board_by_columns<T>(board: &[[T; 9]; 9]) -> [[&T; 9]; 9] {
    let mut columns: [[&T; 9]; 9] = [[&board[0][0]; 9]; 9];
    for y in 0..9 {
        for x in 0..9 {
            columns[y][x] = &board[x][y]
        }
    }
    return columns;
}

pub fn group_board_by_boxes<T>(board: &[[T; 9]; 9]) -> [[&T; 9]; 9] {
    let mut boxes: [[&T; 9]; 9] = [[&board[0][0]; 9]; 9];
    for y in 0..9 {
        for x in 0..9 {
            boxes[(y / 3) * 3 + x / 3][(y % 3) * 3 + x % 3] = &board[y][x]
        }
    }
    return boxes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_board() {
        assert_eq!(
            parse_board("....3..5..8...5..1.592.63..42.5......6.4.2........9..5.46.....3.....78..........4").unwrap(),
            [
                [None, None, None, Some(Number::Four), None, None, None, None, None],
                [None, Some(Number::Eight), Some(Number::Five), Some(Number::Two), Some(Number::Six), None, Some(Number::Four), None, None],
                [None, None, Some(Number::Nine), None, None, None, Some(Number::Six), None, None],
                [None, None, Some(Number::Two), Some(Number::Five), Some(Number::Four), None, None, None, None],
                [Some(Number::Three), None, None, None, None, None, None, None, None],
                [None, Some(Number::Five), Some(Number::Six), None, Some(Number::Two), Some(Number::Nine), None, Some(Number::Seven), None],
                [None, None, Some(Number::Three), None, None, None, None, Some(Number::Eight), None],
                [Some(Number::Five), None, None, None, None, None, None, None, None],
                [None, Some(Number::One), None, None, None, Some(Number::Five), Some(Number::Three), None, Some(Number::Four)]
            ]
        );
    }

    #[test]
    fn test_parse_board_error_invalid_character() {
        assert!(parse_board(
            "......2.6...7....8.5.... .36.1.5.32.....37.....2......9...7......36..4915..81...."
        ).is_err());
    }

    #[test]
    fn test_parse_board_error_not_enough_fields() {
        assert!(parse_board("...4..").is_err());
    }

    #[test]
    fn test_parse_board_error_too_many_fields() {
        assert!(parse_board(
            ".3.79.......6.8....27..5...3...4..9....8.94......6..58....82..118.9.....7.5...6..."
        ).is_err());
    }

    #[test]
    fn test_group_board() {
        let mut board = [[0; 9]; 9];
        for y in 0..9 {
            for x in 0..9 {
                board[y][x] = y * 9 + x
            }
        }
        let columns = group_board_by_columns(&board);
        let boxes = group_board_by_boxes(&board);

        assert_eq!(
            columns,
            [[&0, &9,  &18, &27, &36, &45, &54, &63, &72],
             [&1, &10, &19, &28, &37, &46, &55, &64, &73],
             [&2, &11, &20, &29, &38, &47, &56, &65, &74],
             [&3, &12, &21, &30, &39, &48, &57, &66, &75],
             [&4, &13, &22, &31, &40, &49, &58, &67, &76],
             [&5, &14, &23, &32, &41, &50, &59, &68, &77],
             [&6, &15, &24, &33, &42, &51, &60, &69, &78],
             [&7, &16, &25, &34, &43, &52, &61, &70, &79],
             [&8, &17, &26, &35, &44, &53, &62, &71, &80]]
        );

        assert_eq!(
            boxes,
            [[&0,  &1,  &2,  &9,  &10, &11, &18, &19, &20],
             [&3,  &4,  &5,  &12, &13, &14, &21, &22, &23],
             [&6,  &7,  &8,  &15, &16, &17, &24, &25, &26],
             [&27, &28, &29, &36, &37, &38, &45, &46, &47],
             [&30, &31, &32, &39, &40, &41, &48, &49, &50],
             [&33, &34, &35, &42, &43, &44, &51, &52, &53],
             [&54, &55, &56, &63, &64, &65, &72, &73, &74],
             [&57, &58, &59, &66, &67, &68, &75, &76, &77],
             [&60, &61, &62, &69, &70, &71, &78, &79, &80]]
        );
    }
}