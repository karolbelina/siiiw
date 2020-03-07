use std::cmp::PartialOrd;
use std::fmt::Debug;

pub trait Problem {
    type Solution: ToOwned + Clone;
    type Measure: PartialOrd;

    fn fitness(&self, solution: &Self::Solution) -> Self::Measure;
}
