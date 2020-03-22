use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,
    #[structopt(subcommand)]
    pub problem: Problem,
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
