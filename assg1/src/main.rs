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

    use sf::SolutionFinder;

    let problem = tsp::parser::parse_problem_instance(&config.tsp_path)?;
    let mut random = sf::Random::new(problem, tsp::Randomize);

    use std::time::Duration;

    random.run(Duration::from_secs(2));
    let best = random.get_best_solution();

    println!("best solution: {:?}", best);

    Ok(())
}
