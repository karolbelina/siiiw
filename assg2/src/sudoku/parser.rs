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
}