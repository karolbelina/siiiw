use quicli::prelude::*;
use structopt::StructOpt;

mod problem;
mod sf;
mod tsp;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(flatten)]
    verbosity: Verbosity,
    /// TSP instance file
    #[structopt(long = "input", short = "i", name = "TSP FILE", parse(from_os_str))]
    tsp_path: PathBuf,
}

fn main() -> CliResult {
    let config = Config::from_args();
    config.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

    use problem::Problem;

    let problem = tsp::parser::parse_problem_instance(&config.tsp_path)?;
    // println!("{}", problem.fitness(problem.random_solution()));

    use std::time::Duration;
    use sf::SolutionFinder;

    // let random = sf::Random::new(problem);
    // random.run(Duration::from_secs(2));
    // let best = random.get_best_solution();

    // let greedy = sf::Greedy::new(problem);
    // let evolutionary = sf::Evolutionary

    Ok(())
}
