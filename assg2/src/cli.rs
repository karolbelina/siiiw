use snafu::{Snafu};
use structopt::StructOpt;
use std::path::PathBuf;
use super::csp::solvers::{VariableSelector, ValueSelector};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{:?}", name))]
    InvalidAlgorithmName { name: String },
    #[snafu(display("{:?}", name))]
    InvalidVariableSelectorHeuristicName { name: String },
    #[snafu(display("{:?}", name))]
    InvalidValueSelectorHeuristicName { name: String },
}

use std::fmt;

impl fmt::Debug for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Algorithm::Backtracking => write!(f, "backtracking"),
            Algorithm::ForwardChecking => write!(f, "forward-checking")
        }
    }
}

impl fmt::Debug for VariableSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableSelector::OrderOfDefinition => write!(f, "order of definition"),
            VariableSelector::MostConstrainedVariable => write!(f, "most constrained variable"),
            VariableSelector::Random => write!(f, "random")
        }
    }
}

impl fmt::Debug for ValueSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueSelector::OrderOfDefinition => write!(f, "order of definition"),
            ValueSelector::LeastConstrainingValue => write!(f, "least constraining value"),
            ValueSelector::Random => write!(f, "random")
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,
    /// The algorithm used to find solutions to the problem
    #[structopt(short = "a", name = "ALGORITHM", parse(try_from_str = parse_algorithm))]
    pub algorithm: Algorithm,
    /// The variable selection heuristic
    #[structopt(short = "r", name = "VARIABLE SELECTION HEURISTIC", parse(try_from_str = variable_selection_heuristic))]
    pub variable_selection_heuristic: VariableSelector,
    /// The value selection heuristic
    #[structopt(short = "l", name = "VALUE SELECTION HEURISTIC", parse(try_from_str = value_selection_heuristic))]
    pub value_selection_heuristic: ValueSelector,
    #[structopt(subcommand)]
    pub problem: Problem,
}

pub enum Algorithm {
    Backtracking,
    ForwardChecking,
}

fn parse_algorithm(source: &str) -> std::result::Result<Algorithm, Error> {
    match source {
        "bt" => Ok(Algorithm::Backtracking),
        "fc" => Ok(Algorithm::ForwardChecking),
        _ => Err(Error::InvalidAlgorithmName { name: source.to_owned() })
    }
}

fn variable_selection_heuristic(source: &str) -> std::result::Result<VariableSelector, Error> {
    match source {
        "def" => Ok(VariableSelector::OrderOfDefinition),
        "mcv" => Ok(VariableSelector::MostConstrainedVariable),
        "rand" => Ok(VariableSelector::Random),
        _ => Err(Error::InvalidVariableSelectorHeuristicName { name: source.to_owned() })
    }
}

fn value_selection_heuristic(source: &str) -> std::result::Result<ValueSelector, Error> {
    match source {
        "def" => Ok(ValueSelector::OrderOfDefinition),
        "lcv" => Ok(ValueSelector::LeastConstrainingValue),
        "rand" => Ok(ValueSelector::Random),
        _ => Err(Error::InvalidValueSelectorHeuristicName { name: source.to_owned() })
    }
}

#[derive(Debug, StructOpt)]
pub enum Problem {
    #[structopt(name = "jolka")]
    Jolka {
        /// Jolka puzzle file
        #[structopt(name = "JOLKA PUZZLE FILE", parse(from_os_str))]
        jolka_puzzle_path: PathBuf,
        /// Jolka words file
        #[structopt(name = "JOLKA WORDS FILE", parse(from_os_str))]
        jolka_words_path: PathBuf,
    },
    #[structopt(name = "sudoku")]
    Sudoku {
        /// CSV file containing sudoku boards
        #[structopt(name = "SUDOKU FILE", parse(from_os_str))]
        sudoku_path: PathBuf,
        /// Id of the sudoku board
        #[structopt(name = "SUDOKU ID")]
        sudoku_id: String
    }
}

pub type Result = std::result::Result<(), exitfailure::ExitFailure>;

pub fn setup_env_logger(verbosity: &clap_verbosity_flag::Verbosity)
    -> std::result::Result<(), log::SetLoggerError>
{
    use env_logger::Builder as LoggerBuilder;

    let level_filter = verbosity.log_level().unwrap().to_level_filter();
    LoggerBuilder::new()
        .filter(Some(&env!("CARGO_PKG_NAME").replace("-", "_")), level_filter)
        .filter(None, log::Level::Warn.to_level_filter())
        .try_init()?;
    Ok(())
}
