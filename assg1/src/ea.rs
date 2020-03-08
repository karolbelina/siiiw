use crate::problem::Problem;
use std::borrow::Cow;

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

    fn select<'a>(&self, population: &'a Vec<Individual<Self::Problem>>) -> &'a Individual<Self::Problem>;
}

pub trait Mutate {
    type Problem: Problem;

    fn mutate<'a>(&self, individual: &'a Individual<Self::Problem>) -> Cow<'a, Individual<Self::Problem>>;
}

// pub trait Crossover<P: Problem> {
//     fn crossover(&self, a: &Individual<P>, b: &Individual<P>) -> Individual<P>;
// }

pub struct Evolutionary<'a, P: Problem, I/*, S, C, M*/>
where
    I: Initialize<Problem=P>,
    // S: Select<P>,
    // C: Crossover<P>,
    // M: Mutate<P>
{
    problem: &'a P,
    initialize: &'a I,
    // select: &'a S,
    // crossover: &'a C,
    // mutate: &'a M,
    pop_size: usize,
    generations: usize,
}

impl<P: Problem, I/*, S, C, M*/> Evolutionary<'_, P, I/*, S, C, M*/>
where
    I: Initialize<Problem=P>,
    // S: Select<P>,
    // C: Crossover<P>,
    // M: Mutate<P>
{
    pub fn new<'a>(problem: &'a P, initialize: &'a I/*, select: &'a S, crossover: &'a C, mutate: &'a M*/,
        pop_size: usize, generations: usize) -> Evolutionary<'a, P, I/*, S, C, M*/>
    {
        Evolutionary {
            problem: problem,
            initialize: initialize,
            // select: select,
            // crossover: crossover,
            // mutate: mutate,
            pop_size: pop_size,
            generations: generations,
        }
    }

    pub fn run(&self) {
        let mut current_generation = self.initialize.initialize(self.pop_size);
        for _ in 0..self.generations {
            
        }
    }
}
