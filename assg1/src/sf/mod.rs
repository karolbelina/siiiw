mod random;
mod ea;

pub use random::{Randomize, Random};
pub use ea::Mutate;

use crate::problem::Problem;

pub trait SolutionFinder<P: Problem> {
    fn get_best_solution(&self) -> Option<&P::Solution>;
}
