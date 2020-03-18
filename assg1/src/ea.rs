use crate::problem::Problem;

pub struct Individual<P: Problem> {
    pub genotype: P::Solution,
    pub fitness: P::Measure,
}

impl<P: Problem> Clone for Individual<P> {
    fn clone(&self) -> Individual<P> {
        Individual {
            genotype: self.genotype.clone(),
            fitness: self.fitness.clone(),
        }
    }
}

pub trait Initialize {
    type Problem: Problem;

    fn initialize(&self, pop_size: usize) -> Vec<Individual<Self::Problem>>;
}

pub trait Select {
    type Problem: Problem;

    fn select<'a>(&self, population: &'a Vec<Individual<Self::Problem>>)
        -> &'a Individual<Self::Problem>;
}

pub trait Crossover {
    type Problem: Problem;

    fn crossover<'a>(&self, a: &'a Individual<Self::Problem>, b: &'a Individual<Self::Problem>)
        -> Individual<Self::Problem>;
}

pub trait Mutate {
    type Problem: Problem;

    fn mutate(&self, individual: &mut Individual<Self::Problem>);
}

pub struct Evolutionary<P: Problem, I, S, C, M>
where
    I: Initialize<Problem=P>,
    S: Select<Problem=P>,
    C: Crossover<Problem=P>,
    M: Mutate<Problem=P>
{
    initialize: I,
    select: S,
    crossover: C,
    mutate: M,
    pop_size: usize,
    generations: usize,
}

use crate::log::Log;

impl<P: Problem, I, S, C, M> Evolutionary<P, I, S, C, M>
where
    I: Initialize<Problem=P>,
    S: Select<Problem=P>,
    C: Crossover<Problem=P>,
    M: Mutate<Problem=P>
{
    pub fn new<'a>(initialize: I, select: S, crossover: C, mutate: M,
        pop_size: usize, generations: usize) -> Evolutionary<P, I, S, C, M>
    {
        Evolutionary {
            initialize: initialize,
            select: select,
            crossover: crossover,
            mutate: mutate,
            pop_size: pop_size,
            generations: generations,
        }
    }

    pub fn run(&self, loggers: &mut Vec<&mut dyn Log<(P::Solution, P::Measure)>>) {
        use quicli::prelude::*;
        use std::time::Instant;

        let start = Instant::now();
        info!("started the evolutionary algorithm");

        let mut current_generation = self.initialize.initialize(self.pop_size);
        info!("initialized the population");
        
        for i in 0..self.generations {
            let mut next_generation = Vec::new();
            while next_generation.len() < self.pop_size {
                let parent1 = self.select.select(&current_generation);
                let parent2 = self.select.select(&current_generation);
                let mut offspring = self.crossover.crossover(&parent1, &parent2);
                self.mutate.mutate(&mut offspring);
                
                for logger in loggers.iter_mut() {
                    logger.log(&(
                        offspring.genotype.clone(),
                        offspring.fitness.clone()
                    ));
                }
                next_generation.push(offspring);
            }
            current_generation = next_generation;
            info!("finished generation #{}", i + 1);
        }
        
        let duration = start.elapsed();
        info!("finished the evolutionary algorithm in {:?}", duration);
    }
}
