use super::{CSP, Solve};

pub enum VariableSelector {
    First,
}

impl VariableSelector {
    pub fn select_variable<'a, V>(&self, variables: &'a mut Vec<&mut V>) -> &'a mut V {
        match self {
            Self::First => {
                return variables.first_mut().unwrap();
            }
        }
    }
}

pub struct Backtracking<'a, P: CSP<'a>> {
    problem: &'a mut P,
    variable_selector: &'a VariableSelector,
}

impl<'a, P: CSP<'a>> Backtracking<'a, P> {
    fn backtrack(&'a self, unassigned_variables: Vec<&mut P::Variable>, constraints: &P::Constraints) -> bool {
        use super::{Constraint, Variable};

        for constraint in constraints.clone().into_iter() {
            if !constraint.is_satisfied() {
                return false;
            }
        }
        if unassigned_variables.is_empty() {
            // println!("{:?}", self.problem);
            return false;
        }
        let a = &mut unassigned_variables;
        let variable: &mut P::Variable = self.variable_selector.select_variable(&mut *a);
        for value in self.problem.values() {
            **variable = Some(value);
            let result = self.backtrack(
                unassigned_variables.iter().filter(|x| {
                    **x as *const P::Variable == variable as *const P::Variable
                }).map(|x| *x).collect(),
                constraints
            );
            if result == true {
                return true;
            }
            **variable = None;
        }
        return false;
    }
}

impl<'a, P: CSP<'a>> Solve<'a> for Backtracking<'a, P> {
    fn solve(&'a mut self) -> bool {
        let variables = self.problem.variables();
        self.backtrack(variables, &self.problem.constraints())
    }
}
