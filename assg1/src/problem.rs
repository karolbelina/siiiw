pub trait Problem {
    type Solution: ToOwned + Clone;
    type Measure: BetterThan + Clone;

    fn fitness(&self, solution: &Self::Solution) -> Self::Measure;
}

pub trait BetterThan {
    fn better_than(&self, other: &Self) -> bool;
}
