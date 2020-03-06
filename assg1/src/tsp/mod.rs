mod dm;
pub mod reader;

use dm::DistanceMatrix;

pub struct TSPInstance {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub dimension: usize,
    pub dm: DistanceMatrix,
}

pub struct TSPSolution(Vec<usize>);

use crate::problem::Problem;

impl Problem for TSPInstance {
    type Solution = TSPSolution;
    type Measure = f64;

    fn fitness(&self, solution: TSPSolution) -> f64 {
        use std::f64::INFINITY;

        (0..self.dimension).zip((0..self.dimension).cycle().skip(1))
            .map(|(i, j)| solution.0.get(i)
                .and_then(|a| solution.0.get(j)
                    .and_then(|b| self.dm.get(*a, *b))))
            .sum::<Option<f64>>()
            .unwrap_or(INFINITY)
    }
}