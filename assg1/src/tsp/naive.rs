use super::TSP;

pub struct Random<'a> {
    problem: &'a TSP,
    count: usize,
}
    
use crate::log::Log;
use crate::problem::Problem;

impl Random<'_> {
    pub fn new(problem: &TSP, count: usize) -> Random {
        Random {
            problem: problem,
            count: count,
        }
    }

    pub fn run(&self, loggers: &mut Vec<&mut dyn Log<(Vec<usize>, u32)>>) {
        for _ in 0..self.count {
            let solution = self.next();
            let fitness = self.problem.fitness(&solution);
            for logger in loggers.iter_mut() {
                logger.log(&(solution.clone(), fitness));
            }
        }
    }

    fn next(&self) -> Vec<usize> {
        use rand::seq::SliceRandom;

        let mut genotype: Vec<usize> = (0..self.problem.dimension).collect();
        genotype.shuffle(&mut rand::thread_rng());
        return genotype;
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

    pub fn run(&self, loggers: &mut Vec<&mut dyn Log<(Vec<usize>, u32)>>) {
        for starting_node in 0..self.problem.dimension {
            let solution = self.next(starting_node);
            let fitness = self.problem.fitness(&solution);
            for logger in loggers.iter_mut() {
                logger.log(&(solution.clone(), fitness));
            }
        }
    }

    fn next(&self, starting_node: usize) -> Vec<usize> {
        use std::cmp::Ordering;

        assert!(starting_node < self.problem.dimension, "invalid starting node: {}", starting_node);

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
        return genotype;
    }
}
