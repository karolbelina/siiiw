use std::ops::{Deref, DerefMut};
use std::fmt::Debug;

pub trait CSP<'a> {
    type Value: Copy + Debug + PartialEq;
    type Variable: Deref + DerefMut + Debug + 'a;
    type Values: Iterator<Item=Self::Value>;
    type Variables: Iterator<Item=&'a mut Self::Variable>;

    fn values() -> Self::Values;

    fn variables(&'a mut self) -> Self::Variables;
}
