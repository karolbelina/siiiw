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

    let mut discoverer = tsp::logs::Discoverer::new();
    let mut cohorter = tsp::logs::Cohorter::new(1000, 250);
    for _ in 0..10 {
        let evolutionary = ea::Evolutionary::new(
            ops::initialize::Random::new(&problem),
            ops::select::Tournament::new(15),
            ops::crossover::OX::new(&problem, 0.8),
            ops::mutate::Inversion::new(&problem, 0.1),
            1000,
            250,
        );
        evolutionary.run(&mut vec![&mut discoverer, &mut cohorter]);
        discoverer.carry();
        cohorter.carry();
    }
    discoverer.print();
    if let Some(path) = config.output_path {
        cohorter.dump(&path)?;
    }

    Ok(())
}
