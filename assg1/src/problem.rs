pub trait Problem {
    type Solution: ToOwned + Clone;
    type Measure: Clone;

    fn fitness(&self, solution: &Self::Solution) -> Self::Measure;
}
