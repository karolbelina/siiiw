use structopt::StructOpt;

mod cli;
mod csp;
mod jolka;
mod sudoku;

fn main() -> cli::Result {
    let config = cli::Config::from_args();
    cli::setup_env_logger(&config.verbosity)?;

    use csp::Solve;

    match config.problem {
        cli::Problem::Jolka { jolka_puzzle_path, jolka_words_path } => {
            let jolka = jolka::Jolka::load(&jolka_puzzle_path, &jolka_words_path)?;

            let backtracking = csp::solvers::Backtracking {
                problem: &jolka,
                variable_selector: &csp::solvers::VariableSelector::First,
            };
            backtracking.solve();
        },
        cli::Problem::Sudoku { sudoku_path, sudoku_id } => {
            let sudoku = sudoku::Sudoku::load(&sudoku_path, &sudoku_id)?;

            let backtracking = csp::solvers::Backtracking {
                problem: &sudoku,
                variable_selector: &csp::solvers::VariableSelector::First,
            };
            backtracking.solve();
        }
    }

    Ok(())
}
