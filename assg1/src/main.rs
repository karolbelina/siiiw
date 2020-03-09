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

    use tsp::{parser::parse_problem_instance, ops};

    let problem = parse_problem_instance(&config.tsp_path)?;

    let mut discoverer = tsp::logs::Discoverer::new();
    let random = tsp::naive::Random::new(&problem, 10000);
    random.run(&mut vec![&mut discoverer]);
    println!("{:?}, {:?}", discoverer.get_best_solution(), discoverer.get_best_fitness());

    let mut discoverer = tsp::logs::Discoverer::new();
    let greedy = tsp::naive::Greedy::new(&problem);
    greedy.run(&mut vec![&mut discoverer]);
    println!("{:?}, {:?}", discoverer.get_best_solution(), discoverer.get_best_fitness());

    let mut discoverer = tsp::logs::Discoverer::new();
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

    Ok(())
}
