use crate::problem::Problem;
use super::SolutionFinder;

pub trait Randomize<P: Problem> {
    fn randomize(&self, problem: &P) -> P::Solution;
}

pub struct Random<P, R>
where
    P: Problem,
    R: Randomize<P>
{
    problem: P,
    randomize: R,
    best: Option<(P::Solution, P::Measure)>,
}

use std::time::Duration;

impl<P, R> Random<P, R>
where
    P: Problem,
    R: Randomize<P>
{
    pub fn new(problem: P, randomize: R) -> Random<P, R> {
        Random {
            problem: problem,
            randomize: randomize,
            best: None,
        }
    }

    pub fn run(&mut self, duration: Duration) {
        use quicli::prelude::*;
        use std::time::Instant;

        let start = Instant::now();
        info!("starting the random algorithm");
        while start.elapsed() < duration {
            let solution = self.randomize.randomize(&self.problem);
            let fitness = self.problem.fitness(&solution);

            match &self.best {
                Some((_, best_fitness)) if fitness < *best_fitness => continue,
                _ => self.best = Some((solution, fitness))
            }
        }
        info!("finished the random algorithm");
    }
}

impl<P, R> SolutionFinder<P> for Random<P, R>
where
    P: Problem,
    R: Randomize<P>
{
    fn get_best_solution(&self) -> Option<&P::Solution> {
        self.best.as_ref().map(|(solution, _)| solution)
    }
}
