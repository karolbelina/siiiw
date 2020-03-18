use quicli::prelude::*;
use structopt::StructOpt;

mod ea;
mod tsp;
mod problem;
mod log;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(flatten)]
    verbosity: Verbosity,
    /// TSP instance file
    #[structopt(long = "input", short = "i", name = "TSP FILE", parse(from_os_str))]
    tsp_path: PathBuf,
    /// Cohorter output file
    #[structopt(long = "output", short = "o", name = "OUTPUT FILE", parse(from_os_str))]
    output_path: Option<PathBuf>,
}

fn main() -> CliResult {
    let config = Config::from_args();
    config.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

    use tsp::{parser::parse_problem_instance, ops};

    let problem = parse_problem_instance(&config.tsp_path)?;

    // let mut discoverer = tsp::logs::Discoverer::new();
    // let random = tsp::naive::Random::new(&problem, 10000);
    // random.run(&mut vec![&mut discoverer]);
    // println!("{:?}, {:?}", discoverer.get_best_solution(), discoverer.get_best_fitness());

    // let mut discoverer = tsp::logs::Discoverer::new();
    // let greedy = tsp::naive::Greedy::new(&problem);
    // greedy.run(&mut vec![&mut discoverer]);
    // println!("{:?}, {:?}", discoverer.get_best_solution(), discoverer.get_best_fitness());

    let mut discoverer = tsp::logs::Discoverer::new();
    let mut cohorter = tsp::logs::Cohorter::new(5000, 250);
    for _ in 0..10 {
        let evolutionary = ea::Evolutionary::new(
            ops::initialize::Random::new(&problem),
            ops::select::Tournament::new(6),
            ops::crossover::OX::new(&problem, 1.0),
            ops::mutate::Inversion::new(&problem, 0.03),
            5000,
            250
        );
        evolutionary.run(&mut vec![&mut discoverer, &mut cohorter]);
        println!("{:?}, {:?}", discoverer.get_best_solution(), discoverer.get_best_fitness());
        cohorter.carry();
    }
    if let Some(path) = config.output_path {
        cohorter.dump(&path)?;
    }

    Ok(())
}
