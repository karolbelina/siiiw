use super::{CSP, Solution};
use std::collections::{HashMap, HashSet};

pub struct Order<T> {
    source: Vec<T>,
    forward: usize,
    backward: usize,
}

impl<T> Order<T> {
    pub fn new(source: Vec<T>) -> Order<T> {
        let size = source.len();
        Order {
            source: source,
            forward: 0,
            backward: size - 1,
        }
    }
}

impl<T: Clone> Iterator for Order<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = self.source.get(self.forward);
        self.forward += 1;
        return result.cloned();
    }
}

impl<T: Clone> DoubleEndedIterator for Order<T> {
    fn next_back(&mut self) -> Option<T> {
        let result = self.source.get(self.backward);
        self.backward -= 1;
        return result.cloned();
    }
}

#[derive(Copy, Clone)]
pub enum VariableSelector {
    OrderOfDefinition,
    MostConstrainedVariable,
    Random,
}

impl VariableSelector {
    pub fn variables<'a, P: CSP<'a>>(&self, domains: HashMap<P::Variable, HashSet<P::Value>>)
        -> Order<P::Variable>
    {
        match self {
            Self::OrderOfDefinition => {
                use itertools::Itertools;

                let variables = domains.iter()
                    .sorted_by(|(va, _), (vb, _)| Ord::cmp(&va, &vb))
                    .map(|(a, _)| a.clone())
                    .collect();
                Order::new(variables)
            }
            Self::MostConstrainedVariable => {
                use itertools::Itertools;

                let variables = domains.iter()
                    .sorted_by(|(va, domain_a), (vb, domain_b)| {
                        match Ord::cmp(&domain_a.len(), &domain_b.len()) {
                            std::cmp::Ordering::Equal => Ord::cmp(&va, &vb),
                            o => o
                        }
                    })
                    .map(|(a, _)| a.clone())
                    .collect();
                Order::new(variables)
            },
            Self::Random => {
                use rand::thread_rng;
                use rand::seq::SliceRandom;

                let mut variables: Vec<P::Variable> = domains.iter()
                    .map(|(a, _)| a.clone())
                    .collect();
                variables.shuffle(&mut thread_rng());
                Order::new(variables)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum ValueSelector {
    OrderOfDefinition,
    LeastConstrainingValue,
    Random,
}

impl ValueSelector {
    pub fn values<'a, P: CSP<'a>>(
        &self,
        variable: &P::Variable,
        domains: &HashMap<P::Variable, HashSet<P::Value>>,
        _: &P::Constraints
    ) -> Order<P::Value>
    {
        match self {
            Self::OrderOfDefinition => {
                use itertools::Itertools;

                let values = domains.get(variable).unwrap().iter()
                    .map(|a| a.clone())
                    .sorted_by(|a, b| Ord::cmp(&a, &b))
                    .collect();
                Order::new(values)
            },
            Self::LeastConstrainingValue => {
                unimplemented!()
            },
            Self::Random => {
                use rand::thread_rng;
                use rand::seq::SliceRandom;

                let mut values: Vec<P::Value> = domains.get(variable).unwrap().iter()
                    .map(|a| a.clone())
                    .collect();
                values.shuffle(&mut thread_rng());
                Order::new(values)
            }
        }
    }
}

use std::time::Instant;

#[derive(Clone)]
struct Statistics {
    visited_nodes: usize,
    backtracks: usize,
    solutions_found: usize,
    start_time: Instant,
}

pub fn backtracking<'a, P: CSP<'a>>(
    problem: &'a P,
    variable_selector: VariableSelector,
    value_selector: ValueSelector
) -> HashSet<P::Solution> {
    use log::{info, warn};
    use super::Constraint;

    fn backtrack<'a, P: CSP<'a>>(
        problem: &'a P, variable_selector: VariableSelector, value_selector: ValueSelector,
        domains: HashMap<P::Variable, HashSet<P::Value>>, assignments: HashMap<P::Variable, P::Value>,
        constraints: &P::Constraints, statistics: &mut Statistics
    ) -> HashSet<P::Solution> {
        if let Some(variable) = variable_selector.variables::<P>(domains.clone()).next() {
            let mut solutions = HashSet::new();
            for value in value_selector.values::<P>(&variable, &domains, &constraints).rev() {
                statistics.visited_nodes += 1;
                let mut new_domains = domains.clone();
                let mut new_assignments = assignments.clone();
                new_domains.remove(&variable);
                new_assignments.insert(variable.clone(), value.clone());
                if constraints.clone().into_iter().all(|constraint| {
                    constraint.is_satisfied(&new_assignments)
                }) {
                    solutions.extend(backtrack::<P>(
                        problem, variable_selector, value_selector, new_domains,
                        new_assignments, constraints, statistics
                    ));
                } else {
                    statistics.backtracks += 1;
                }
            }
            statistics.backtracks += 1;
            return solutions;
        } else {
            if statistics.solutions_found == 0 {
                info!(
                    "First solution found in {:.2?}, after {} visited nodes and {} backtracks",
                    statistics.start_time.elapsed(), statistics.visited_nodes, statistics.backtracks
                );
            }
            statistics.solutions_found += 1;
            statistics.backtracks += 1;
            return [P::Solution::construct(problem, &assignments)].iter().cloned().collect();
        }
    }

    let mut statistics = Statistics {
        visited_nodes: 0,
        backtracks: 0,
        solutions_found: 0,
        start_time: Instant::now(),
    };
    info!("Initialized the backtracking method");
    let solutions = backtrack(
        problem, variable_selector, value_selector, problem.domains(),
        HashMap::new(), &problem.constraints(), &mut statistics
    );
    statistics.backtracks -= 1; // function exit
    info!(
        "Finished the algorithm in {:.2?}, after {} visited nodes and {} backtracks",
        statistics.start_time.elapsed(), statistics.visited_nodes, statistics.backtracks
    );
    match solutions.len() {
        0 => warn!("0 solutions found"),
        1 => info!("1 solution found"),
        n => info!("{} solutions found", n)
    }
    return solutions;
}

pub fn forward_checking<'a, P: CSP<'a>>(
    problem: &'a P,
    variable_selector: VariableSelector,
    value_selector: ValueSelector
) -> HashSet<P::Solution> {
    use log::{info, warn};
    use super::Constraint;

    fn backtrack<'a, P: CSP<'a>>(
        problem: &'a P, variable_selector: VariableSelector, value_selector: ValueSelector,
        domains: HashMap<P::Variable, HashSet<P::Value>>, assignments: HashMap<P::Variable, P::Value>,
        constraints: &P::Constraints, statistics: &mut Statistics
    ) -> HashSet<P::Solution> {
        if let Some(variable) = variable_selector.variables::<P>(domains.clone()).next() {
            let mut solutions = HashSet::new();
            for value in value_selector.values::<P>(&variable, &domains, &constraints).rev() {
                statistics.visited_nodes += 1;
                let mut new_domains = domains.clone();
                let mut new_assignments = assignments.clone();
                new_domains.remove(&variable);
                new_assignments.insert(variable.clone(), value.clone());
                for constraint in constraints.clone().into_iter() {
                    constraint.prune(&mut new_domains, &variable, &value);
                }
                if new_domains.values().all(|domain| {
                    !domain.is_empty()
                }) {
                    solutions.extend(backtrack::<P>(
                        problem, variable_selector, value_selector, new_domains,
                        new_assignments, constraints, statistics
                    ));
                } else {
                    statistics.backtracks += 1;
                }
            }
            statistics.backtracks += 1;
            return solutions;
        } else {
            if statistics.solutions_found == 0 {
                info!(
                    "First solution found in {:.2?}, after {} visited nodes and {} backtracks",
                    statistics.start_time.elapsed(), statistics.visited_nodes, statistics.backtracks
                );
            }
            statistics.solutions_found += 1;
            statistics.backtracks += 1;
            return [P::Solution::construct(problem, &assignments)].iter().cloned().collect();
        }
    }

    let mut statistics = Statistics {
        visited_nodes: 0,
        backtracks: 0,
        solutions_found: 0,
        start_time: Instant::now(),
    };
    info!("Initialized the forward-checking method");
    let solutions = backtrack(
        problem, variable_selector, value_selector, problem.domains(),
        HashMap::new(), &problem.constraints(), &mut statistics
    );
    statistics.backtracks -= 1; // function exit
    info!(
        "Finished the algorithm in {:.2?}, after {} visited nodes and {} backtracks",
        statistics.start_time.elapsed(), statistics.visited_nodes, statistics.backtracks
    );
    match solutions.len() {
        0 => warn!("0 solutions found"),
        1 => info!("1 solution found"),
        n => info!("{} solutions found", n)
    }
    return solutions;
}
