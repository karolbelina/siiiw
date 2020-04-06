use structopt::StructOpt;

mod cli;
mod csp;
mod jolka;
mod sudoku;

fn main() -> cli::Result {
    let config = cli::Config::from_args();
    cli::setup_env_logger(&config.verbosity)?;

    use log::info;

    match config.problem {
        cli::Problem::Jolka { jolka_puzzle_path, jolka_words_path } => {
            let jolka = jolka::Jolka::load(&jolka_puzzle_path, &jolka_words_path)?;

            let solutions = match config.algorithm {
                cli::Algorithm::Backtracking => csp::solvers::backtracking(
                    &jolka,
                    config.variable_selection_heuristic,
                    config.value_selection_heuristic,
                ),
                cli::Algorithm::ForwardChecking => csp::solvers::forward_checking(
                    &jolka,
                    config.variable_selection_heuristic,
                    config.value_selection_heuristic,
                )
            };
            info!("Displaying the solutions");
            for solution in solutions {
                println!("{:?}\n", solution);
            }
        },
        cli::Problem::Sudoku { sudoku_path, sudoku_id } => {
            let sudoku = sudoku::Sudoku::load(&sudoku_path, &sudoku_id)?;

            let solutions = match config.algorithm {
                cli::Algorithm::Backtracking => csp::solvers::backtracking(
                    &sudoku,
                    config.variable_selection_heuristic,
                    config.value_selection_heuristic,
                ),
                cli::Algorithm::ForwardChecking => csp::solvers::forward_checking(
                    &sudoku,
                    config.variable_selection_heuristic,
                    config.value_selection_heuristic,
                )
            };
            info!("Displaying the solutions");
            for solution in solutions {
                println!("{:?}\n", solution);
            }
        }
    }

    Ok(())
}
