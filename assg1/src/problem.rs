use std::cmp::PartialOrd;
use std::fmt::Debug;

pub trait Problem {
    type Solution: ToOwned + Clone + Debug;
    type Measure: PartialOrd + Debug;

    fn fitness(&self, solution: &Self::Solution) -> Self::Measure;
}
