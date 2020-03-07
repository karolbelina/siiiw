use crate::problem::Problem;

pub mod ea;

pub trait SolutionFinder<P: Problem> {
    fn get_best_solution(&self) -> Option<&P::Solution>;
}

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
    fn new(problem: P, randomize: R) -> Random<P, R> {
        Random {
            problem: problem,
            randomize: randomize,
            best: None,
        }
    }

    fn run(&self, duration: Duration) {
        use std::time::Instant;

        let start = Instant::now();
        while start.elapsed() < duration {
            let solution = self.randomize.randomize(&self.problem);
        }
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
