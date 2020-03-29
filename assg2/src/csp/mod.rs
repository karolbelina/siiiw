pub mod solvers;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::{Debug, Display};

pub trait Constraint<'a, P: CSP<'a>> {
    fn is_satisfied(&self, env: &HashMap<P::Variable, P::Value>) -> bool;
}

pub trait Solution<'a, P: CSP<'a>> {
    fn construct(problem: &P, env: &HashMap<P::Variable, P::Value>) -> Self;
}

pub trait CSP<'a>: Sized {
    type Value: Clone + Debug + 'a;
    type Values: Iterator<Item=Self::Value>;
    type Variable: Eq + Hash + Copy + Clone + Debug + 'a;
    type Constraint: Constraint<'a, Self>;
    type Constraints: IntoIterator<Item=Self::Constraint> + Clone + Debug;
    type Solution: Solution<'a, Self> + Clone + Eq + Hash + Display;

    fn constraints(&'a self) -> Self::Constraints;

    fn variables(&'a self) -> Vec<&Self::Variable>;

    fn values(&'a self) -> Self::Values;

    fn initial_assignments(&'a self) -> HashMap<Self::Variable, Self::Value>;
}

pub trait Solve<'a, P: CSP<'a>> {
    fn solve(&'a self) -> HashSet<P::Solution>;
}
