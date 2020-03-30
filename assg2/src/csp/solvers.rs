use super::{CSP, Solve, Solution};
use std::collections::{HashMap, HashSet};

pub struct Order<T> {
    source: Vec<T>,
    order: Vec<usize>,
    current: usize,
}

impl<'a, T: Clone> Iterator for Order<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = self.order.get(self.current)
            .map(|index| self.source.get(*index).unwrap().clone());
        self.current += 1;
        return result;
    }
}

pub enum VariableSelector {
    First,
    Random,
}

impl VariableSelector {
    pub fn variables<'a, P: CSP<'a>, I>(&self, source: I)
        -> Order<&'a P::Variable>
    where
        I: Iterator<Item=&'a P::Variable>
    {
        let source_vec: Vec<&P::Variable> = source.collect();
        match self {
            Self::First => {
                Order {
                    order: (0..source_vec.len()).collect(),
                    source: source_vec,
                    current: 0,
                }
            },
            Self::Random => {
                use rand::thread_rng;
                use rand::seq::SliceRandom;

                let mut order: Vec<usize> = (0..source_vec.len()).collect();
                order.shuffle(&mut thread_rng());
                Order {
                    order: order,
                    source: source_vec,
                    current: 0,
                }
            }
        }
    }
}

pub enum ValueSelector {
    First,
    Random,
}

impl ValueSelector {
    pub fn values<'a, P: CSP<'a>, I>(&self, source: I)
        -> Order<P::Value>
    where
        I: Iterator<Item=P::Value>
    {
        let source_vec: Vec<P::Value> = source.collect();
        match self {
            Self::First => {
                Order {
                    order: (0..source_vec.len()).collect(),
                    source: source_vec,
                    current: 0,
                }
            },
            Self::Random => {
                use rand::thread_rng;
                use rand::seq::SliceRandom;

                let mut order: Vec<usize> = (0..source_vec.len()).collect();
                order.shuffle(&mut thread_rng());
                Order {
                    order: order,
                    source: source_vec,
                    current: 0,
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

use std::time::Instant;

impl<'a, P: CSP<'a>> Backtracking<'a, P> {
    fn backtrack(
        &'a self,
        unassigned_variables: Vec<&'a P::Variable>,
        assignments: &HashMap<P::Variable, P::Value>,
        constraints: &P::Constraints,
        solutions_found: &mut usize, visited_nodes: &mut usize,
        backtracks: &mut usize, start_time: &Instant
    ) -> HashSet<P::Solution> {
        use super::Constraint;
        use log::info;

        if let Some(variable) = self.variable_selector.variables::<P, _>(unassigned_variables.iter().cloned()).next() {
            let mut solutions = HashSet::new();

            'values: for value in self.value_selector.values::<P, _>(self.problem.values()) {
                let mut new_assignments = assignments.clone();
                new_assignments.insert(*variable, value);
                *visited_nodes += 1;

                for constraint in constraints.clone().into_iter() {
                    if !constraint.is_satisfied(&new_assignments) {
                        continue 'values;
                    }
                }

                let new_solutions = self.backtrack(
                    unassigned_variables.iter().filter(|x| **x != variable).map(|x| *x).collect(),
                    &new_assignments, constraints, solutions_found, visited_nodes, backtracks, &start_time
                );
                *solutions_found += new_solutions.len();
                solutions.extend(new_solutions);
            }

            *backtracks += 1;
            return solutions;
        }

        if *solutions_found == 0 {
            info!(
                "First solution found in {:.2?}, after {} visited nodes and {} backtracks",
                start_time.elapsed(), visited_nodes, backtracks
            );
        }

        *backtracks += 1;
        return [P::Solution::construct(self.problem, assignments)].iter().cloned().collect();
    }
}

impl<'a, P: CSP<'a>> Solve<'a, P> for Backtracking<'a, P> {
    fn solve(&'a self) -> HashSet<P::Solution> {
        use log::{info, warn};
    
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
            assignments.len(), unassigned_variables.len()
        );
        let mut solutions_found = 0;
        let mut visited_nodes = 0;
        let mut backtracks = 0;
        let start_time = Instant::now();
        let solutions = self.backtrack(
            unassigned_variables, &assignments, &constraints,
            &mut solutions_found, &mut visited_nodes, &mut backtracks, &start_time
        );
        info!(
            "Finished the algorithm in {:.2?}, after {} visited nodes and {} backtracks",
            start_time.elapsed(), visited_nodes, backtracks
        );
        match solutions.len() {
            0 => warn!("0 solutions found"),
            1 => info!("1 solution found"),
            n => info!("{} solutions found", n)
        }
        return solutions;
    }
}
