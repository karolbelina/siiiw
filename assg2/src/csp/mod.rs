pub mod solvers;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;

pub trait Constraint<'a, P: CSP<'a>> {
    fn is_satisfied(&self, env: &HashMap<P::Variable, P::Value>) -> bool;

    fn prune(
        &self,
        domains: &mut HashMap<P::Variable, HashSet<P::Value>>,
        variable: &P::Variable,
        value: &P::Value
    );
}

pub trait Solution<'a, P: CSP<'a>> {
    fn construct(problem: &P, env: &HashMap<P::Variable, P::Value>) -> Self;
}

pub trait CSP<'a>: Sized {
    type Value: Eq + Hash + Clone + Debug + Ord + 'a;
    type Variable: Eq + Hash + Clone + Debug + Ord + 'a;
    type Constraint: Constraint<'a, Self>;
    type Constraints: IntoIterator<Item=Self::Constraint> + Clone + Debug;
    type Solution: Solution<'a, Self> + Clone + Eq + Hash;

    fn constraints(&'a self) -> Self::Constraints;

    fn domains(&'a self) -> HashMap<Self::Variable, HashSet<Self::Value>>;
}
