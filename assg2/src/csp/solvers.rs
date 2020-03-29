use super::{CSP, Solve, Solution};
use std::collections::{HashMap, HashSet};

pub struct Order<T, O: Iterator<Item=usize>> {
    source: Vec<T>,
    order: O,
}

impl<'a, T: Clone, O: Iterator<Item=usize>> Iterator for Order<T, O> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.order.next().map(|index| self.source.get(index).unwrap().clone())
    }
}

pub enum VariableSelector {
    First,
}

impl VariableSelector {
    pub fn variables<'a, P: CSP<'a>, I>(&self, source: I)
        -> Order<&'a P::Variable, std::ops::Range<usize>>
    where
        I: Iterator<Item=&'a P::Variable>
    {
        match self {
            Self::First => {
                let source_vec: Vec<&P::Variable> = source.collect();
                Order {
                    order: (0..source_vec.len()),
                    source: source_vec,
                }
            }
        }
    }
}

pub enum ValueSelector {
    First,
}

impl ValueSelector {
    pub fn values<'a, P: CSP<'a>, I>(&self, source: I)
        -> Order<P::Value, std::ops::Range<usize>>
    where
        I: Iterator<Item=P::Value>
    {
        match self {
            Self::First => {
                let source_vec: Vec<P::Value> = source.collect();
                Order {
                    order: (0..source_vec.len()),
                    source: source_vec,
                }
            }
        }
    }
}

pub struct Backtracking<'a, P: CSP<'a>> {
    pub problem: &'a P,
    pub variable_selector: &'a VariableSelector,
    pub value_selector: &'a ValueSelector,
}

impl<'a, P: CSP<'a>> Backtracking<'a, P> {
    fn backtrack(
        &'a self,
        unassigned_variables: Vec<&'a P::Variable>,
        assignments: &HashMap<P::Variable, P::Value>,
        constraints: &P::Constraints
    ) -> HashSet<P::Solution> {
        use super::Constraint;
        use log::info;

        if let Some(variable) = self.variable_selector.variables::<P, _>(unassigned_variables.iter().cloned()).next() {
            let mut solutions = HashSet::new();
            'values: for value in self.value_selector.values::<P, _>(self.problem.values()) {
                let mut new_assignments = assignments.clone();
                new_assignments.insert(*variable, value);
                for constraint in constraints.clone().into_iter() {
                    if !constraint.is_satisfied(&new_assignments) {
                        continue 'values;
                    }
                }
                let new_solutions = self.backtrack(
                    unassigned_variables.iter().filter(|x| **x != variable).map(|x| *x).collect(),
                    &new_assignments,
                    constraints
                );
                solutions.extend(new_solutions);
            }
            return solutions;
        }
        info!("New solution found");
        return [P::Solution::construct(self.problem, assignments)].iter().cloned().collect();
    }
}

impl<'a, P: CSP<'a>> Solve<'a, P> for Backtracking<'a, P> {
    fn solve(&'a self) -> HashSet<P::Solution> {
        use log::info;

        let assignments = self.problem.initial_assignments();
        let mut unassigned_variables = Vec::new();
        for variable in self.problem.variables() {
            if !assignments.contains_key(variable) {
                unassigned_variables.push(variable)
            }
        }
        let constraints = self.problem.constraints();
        info!(
            "Initialized the backtracking method with {}/{} assigned variables",
            assignments.len(),
            unassigned_variables.len(),
        );
        self.backtrack(unassigned_variables, &assignments, &constraints)
    }
}
