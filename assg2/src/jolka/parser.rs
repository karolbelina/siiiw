use snafu::{ResultExt, Snafu};
use super::{Board, Row, State};

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
    return source.split_whitespace().enumerate().map(|(i, line)| -> Result<Row> {
        line.chars().enumerate().map(|(j, c)| -> Result<State> {
            match c {
                '_' => Ok(State::On),
                '#' => Ok(State::Off),
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_board() {
        use super::{parse_board, State::On, State::Off};

        assert_eq!(
            parse_board("___##__#\n#____#_#\n__#___##").unwrap(),
            vec!(
                vec!(On, On, On, Off, Off, On, On, Off),
                vec!(Off, On, On, On, On, Off, On, Off),
                vec!(On, On, Off, On, On, On, Off, Off),
            )
        );
    }

    #[test]
    fn test_parse_board_error() {
        use super::{parse_board};

        assert!(parse_board("___#\n##;_").is_err());
    }

    #[test]
    fn test_parse_words() {
        use super::{parse_words};

        assert_eq!(
            parse_words("ABC\nTEST\nIJK"),
            vec!("ABC", "TEST", "IJK")
        )
    }
}
