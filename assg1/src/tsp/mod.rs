use quicli::prelude::*;

mod dm;
pub mod parser;
pub mod ops;

use dm::DistanceMatrix;

pub struct TSP {
    pub name: Option<String>,
    dimension: usize,
    dm: DistanceMatrix,
}

use crate::problem::Problem;

impl Problem for TSP {
    type Solution = Vec<usize>;
    type Measure = f64;

    fn fitness(&self, solution: &Vec<usize>) -> f64 {
        use std::f64::INFINITY;

        (0..self.dimension)
            .zip((0..self.dimension).cycle().skip(1))
            .map(|(i, j)| solution.get(i)
                .and_then(|a| solution.get(j)
                    .and_then(|b| self.dm.get(*a, *b))))
            .sum::<Option<f64>>()
            .unwrap_or(INFINITY)
    }
}

pub struct Random<'a> {
    problem: &'a TSP,
}

impl Random<'_> {
    pub fn new(problem: &TSP) -> Random {
        Random {
            problem: problem,
        }
    }

    pub fn next(&self) -> Result<Vec<usize>, Error> {
        use rand::seq::SliceRandom;

        let mut genotype: Vec<usize> = (0..self.problem.dimension).collect();
        genotype.shuffle(&mut rand::thread_rng());
        return Ok(genotype);
    }
}

pub struct Greedy<'a> {
    problem: &'a TSP,
}

impl Greedy<'_> {
    pub fn new(problem: &TSP) -> Greedy {
        Greedy {
            problem: problem,
        }
    }

    pub fn next(&self, starting_node: usize) -> Result<Vec<usize>, Error> {
        use std::cmp::Ordering;

        if starting_node >= self.problem.dimension {
            return Err(format_err!("invalid starting node: {}", starting_node));
        }
        let mut genotype: Vec<usize> = vec![starting_node];
        let mut current_node: usize = starting_node;
        for _ in 0..self.problem.dimension - 1 {
            let nearest_node: usize = self.problem.dm.get_adjacent(current_node)
                .iter()
                .filter(|(i, _)| !genotype.contains(i))
                .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal))
                .unwrap().0;
            genotype.push(nearest_node);
            current_node = nearest_node;
        }
        return Ok(genotype);
    }
}
