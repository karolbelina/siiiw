pub mod solvers;

use std::ops::{Deref, DerefMut};

pub trait Constraint {
    fn is_satisfied(&self) -> bool;
}

pub trait Variable<V>: Deref<Target=Option<V>> + DerefMut {}

pub trait CSP<'a> {
    type Value;
    type Values: Iterator<Item=Self::Value>;
    type Variable: crate::csp::Variable<Self::Value>;
    type Constraint: Constraint;
    type Constraints: IntoIterator<Item=Self::Constraint> + Clone;

    fn constraints(&'a self) -> Self::Constraints;

    fn variables(&'a mut self) -> Vec<&mut Self::Variable>;

    fn values(&'a self) -> Self::Values;
}

pub trait Solve<'a> {
    fn solve(&'a self) -> bool;
}
