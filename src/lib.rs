mod optimizer;
pub mod parse_arguments;
mod parse_dimacs;

use parse_arguments::InputArguments;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Formula {
    pub clauses: Vec<Clause>,
    pub number_of_literals: usize,
}

#[derive(Debug, PartialEq)]
pub struct Clause {
    pub literals: HashMap<usize, bool>,
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub literals: Vec<bool>,
}

impl Solution {
    /// This returns the ratio of clauses that the solution
    /// satisfies
    pub fn evaluate(&self, formula: &Formula) -> f32 {
        let mut num_satisfied_clauses = 0;

        for clause in formula.clauses.iter() {
            if self.satisfies_clause(clause) {
                num_satisfied_clauses += 1;
            }
        }

        num_satisfied_clauses as f32 / formula.clauses.len() as f32
    }

    /// Returns true if at least one of the literals in the clause
    /// has the same value as its matching literal in the solution.
    /// Returns false otherwise
    pub fn satisfies_clause(&self, clause: &Clause) -> bool {
        for (literal_number, literal_value) in clause.literals.iter() {
            let associated_solution_value = self
                .literals
                .get(*literal_number)
                .expect("The solution does not contain a value for all literals");
            if literal_value == associated_solution_value {
                return true;
            }
        }
        false
    }
}

pub fn run(args: InputArguments) {
    let formula = parse_dimacs::parse_dimacs_formula_from_file(&args.formula_path);
    let best_solution = optimizer::optimize(
        &formula,
        args.population_size,
        args.maximum_number_of_generations,
    );

    let best_fitness = best_solution.evaluate(&formula);
    println!("The best solution is {:?}", best_solution);
    println!("It has a fitness of {best_fitness}");
}
