use crate::problem::Problem;
use std::borrow::Cow;

// pub trait Initialize<P: Problem> {
//     fn initialize() -> Vec<P::Solution>;
// }

// pub trait Select<P: Problem> {
//     fn select(&self, population: &Vec<P::Solution>) -> P::Solution;
// }

pub trait Mutate<P: Problem> {
    fn mutate<'a>(&self, individual: &'a P::Solution) -> Cow<'a, P::Solution>;
}

// pub trait Crossover<P: Problem> {
//     fn crossover(&self, a: &P::Solution, b: &P::Solution) -> P::Solution;
// }

// pub struct Evolutionary<P: Problem> {
//     initialize: dyn Initialize<P>,
//     select: dyn Select<P>,
//     crossover: dyn Crossover<P>,
//     mutate: dyn Mutate<P>,
// }

// use super::SolutionFinder;

// impl<P: Problem> SolutionFinder<P> for Evolutionary<P> {

// }
