use quicli::prelude::*;
use structopt::StructOpt;

mod ea;
mod tsp;
mod problem;
mod logger;

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

    use tsp::{parser, ops, TSP};
    // use sf::SolutionAggregator;
    // use problem::Problem;

    let problem = parser::parse_problem_instance(&config.tsp_path)?;
    let mut discoverer = logger::Discoverer::<TSP>::new();
    // let random = tsp::Random::new(&problem);
    // let greedy = tsp::Greedy::new(&problem);
    // println!("{:?}", random.next());
    // println!("{:?}", greedy.next(4));

    let evolutionary = ea::Evolutionary::new(
        ops::initialize::Random::new(&problem),
        ops::select::Tournament::new(5),
        ops::crossover::OX::new(&problem, 0.7),
        ops::mutate::Swap::new(&problem, 0.01),
        1000,
        10
    );
    evolutionary.run(&mut vec![&mut discoverer]);
    
    println!("{:?}, {:?}", discoverer.get_best_solution(), discoverer.get_best_fitness());

    // random.run();

    // let best = random.get_best_solution();

    // println!("best solution: {:?} (fitness = {})", best, problem.fitness(best.unwrap()).0);

    Ok(())
}
