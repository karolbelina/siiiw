pub trait Problem {
    type Solution: ToOwned + Clone;
    type Measure: std::cmp::PartialOrd;

    fn fitness(&self, solution: Self::Solution) -> Self::Measure;
}
