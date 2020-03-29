use super::{CSP, Solve, Solution};
use std::collections::HashMap;

pub enum VariableSelector {
    First,
}

impl VariableSelector {
    pub fn select_variable<'a, V>(&self, variables: &'a Vec<&V>) -> &'a V {
        match self {
            Self::First => {
                return variables.first().unwrap();
            }
        }
    }
}

pub struct Backtracking<'a, P: CSP<'a>> {
    pub problem: &'a P,
    pub variable_selector: &'a VariableSelector,
}

impl<'a, P: CSP<'a>> Backtracking<'a, P> {
    fn backtrack(
        &'a self,
        unassigned_variables: Vec<&P::Variable>,
        assignments: &HashMap<P::Variable, P::Value>,
        constraints: &P::Constraints
    ) -> bool {
        use super::{Constraint};
        use log::info;

        for constraint in constraints.clone().into_iter() {
            if !constraint.is_satisfied(&assignments) {
                return false;
            }
        }
        if unassigned_variables.is_empty() {
            info!("New solution found:\n{}", P::Solution::construct(self.problem, assignments));
            return false;
        }
        let variable: &P::Variable = self.variable_selector.select_variable(&unassigned_variables);
        for value in self.problem.values() {
            let mut new_assignments = assignments.clone();
            new_assignments.insert(*variable, value);
            let result = self.backtrack(
                unassigned_variables.iter().filter(|x| **x != variable).map(|x| *x).collect(),
                &new_assignments,
                constraints
            );
            if result == true {
                return true;
            }
            new_assignments.remove(variable);
        }
        return false;
    }
}

impl<'a, P: CSP<'a>> Solve<'a> for Backtracking<'a, P> {
    fn solve(&'a self) -> bool {
        // use log::info;

        let assignments = self.problem.initial_assignments();
        let mut unassigned_variables = Vec::new();
        for variable in self.problem.variables() {
            if !assignments.contains_key(variable) {
                unassigned_variables.push(variable)
            }
        }
        let constraints = self.problem.constraints();
        // info!("Initialized the backtracking:\n{:?}\n{:?}", unassigned_variables, assignments);
        self.backtrack(unassigned_variables, &assignments, &constraints)
    }
}
