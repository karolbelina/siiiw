mod dm;
pub mod logs;
pub mod naive;
pub mod ops;
pub mod parser;

use dm::DistanceMatrix;

pub struct TSP {
    pub name: Option<String>,
    dimension: usize,
    dm: DistanceMatrix,
}

use crate::problem::Problem;

impl Problem for TSP {
    type Solution = Vec<usize>;
    type Measure = u32;

    fn fitness(&self, solution: &Vec<usize>) -> u32 {
        use std::u32;

        (0..self.dimension)
            .zip((0..self.dimension).cycle().skip(1))
            .map(|(i, j)| solution.get(i)
                .and_then(|a| solution.get(j)
                    .and_then(|b| self.dm.get(*a, *b))))
            .sum::<Option<u32>>()
            .unwrap_or(u32::MAX)
    }
}
