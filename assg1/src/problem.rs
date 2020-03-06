pub trait Problem {
    type Solution;
    type Measure: std::cmp::PartialOrd;

    fn fitness(&self, solution: Self::Solution) -> Self::Measure;
}

pub trait Individual {
    
}
