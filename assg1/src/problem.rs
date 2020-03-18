pub trait Problem {
    type Solution: Clone;
    type Measure: Clone;

    fn fitness(&self, solution: &Self::Solution) -> Self::Measure;
}
