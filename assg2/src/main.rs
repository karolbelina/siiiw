use structopt::StructOpt;

mod cli;
mod csp;
mod jolka;
mod sudoku;

fn main() -> cli::Result {
    let config = cli::Config::from_args();
    cli::setup_env_logger(&config.verbosity)?;

    use csp::CSP;

    match config.problem {
        cli::Problem::Jolka { jolka_puzzle_path, jolka_words_path } => {
            use jolka::parser::{read_board, read_words};

            let board = read_board(&jolka_puzzle_path)?;
            let words = read_words(&jolka_words_path)?;
            println!("{:?}", board);
            println!("{:?}", words);
        },
        cli::Problem::Sudoku { sudoku_path, sudoku_id } => {
            let mut sudoku = sudoku::Sudoku::load(&sudoku_path, &sudoku_id)?;
            
            fn solve<'a, P: CSP<'a>>(problem: &'a mut P) {
                for variable in problem.variables() {
                    for value in P::values() {
                        println!("{:?} {:?}", variable, value);
                    }
                }
            }

            solve(&mut sudoku);
        }
    }

    Ok(())
}
