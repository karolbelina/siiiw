extern crate rand;

mod dm;
pub mod parser;

use dm::DistanceMatrix;

pub struct TSP {
    pub name: Option<String>,
    dimension: usize,
    dm: DistanceMatrix,
}

use crate::problem::Problem;
use std::cmp::Reverse;

impl Problem for TSP {
    type Solution = Vec<usize>;
    type Measure = Reverse<f64>;

    fn fitness(&self, solution: &Vec<usize>) -> Reverse<f64> {
        use std::f64::INFINITY;

        let fitness = (0..self.dimension)
            .zip((0..self.dimension).cycle().skip(1))
            .map(|(i, j)| solution.get(i)
                .and_then(|a| solution.get(j)
                    .and_then(|b| self.dm.get(*a, *b))))
            .sum::<Option<f64>>()
            .unwrap_or(INFINITY);
        
        Reverse(fitness)
    }
}

use crate::sf::ea::{Mutate};
use std::borrow::Cow;

struct Swap {
    probability: f64,
}

impl Mutate<TSP> for Swap {
    fn mutate<'a>(&self, individual: &'a Vec<usize>) -> Cow<'a, Vec<usize>> {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0, 1.0) < self.probability {
            if individual.len() > 1 {
                use rand::distributions::{Distribution, Uniform};

                let between = Uniform::from(0..individual.len());
                let first = between.sample(&mut rng);
                let second = between.sample(&mut rng);

                if first != second {
                    let mut mutated_individual = individual.clone();
                    mutated_individual.swap(first, second);
                    return Cow::Owned(mutated_individual);
                }
            }
        }

        return Cow::Borrowed(individual);
    }
}

use crate::sf;

pub struct Randomize;

impl sf::Randomize<TSP> for Randomize {
    fn randomize(&self, problem: &TSP) -> Vec<usize> {
        use rand::seq::SliceRandom;

        let mut vec: Vec<usize> = (0..problem.dimension).collect();
        vec.shuffle(&mut rand::thread_rng());
        return vec;
    }
}
