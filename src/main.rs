use std::collections::HashMap;

#[derive(Debug)]
struct Formula {
    clauses: Vec<Clause>,
}

#[derive(Debug)]
struct Clause {
    literals: HashMap<usize, bool>,
}

#[derive(Debug)]
struct Solution {
    literals: Vec<bool>,
}

impl Solution {
    /// This returns the ratio of clauses that the solution
    /// satisfies
    fn evaluate(&self, formula: &Formula) -> f32 {
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
    fn satisfies_clause(&self, clause: &Clause) -> bool {
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

fn main() {
    let mut first_clause_values = HashMap::new();
    first_clause_values.insert(0, true);
    first_clause_values.insert(1, true);
    first_clause_values.insert(2, true);

    let mut second_clause_values = HashMap::new();
    second_clause_values.insert(0, false);
    second_clause_values.insert(1, false);
    let f = Formula {
        clauses: vec![
            Clause {
                literals: first_clause_values,
            },
            Clause {
                literals: second_clause_values,
            },
        ],
    };

    let solution = Solution {
        literals: vec![false, false, false],
    };

    println!("The formulat is {:#?}", f);
    println!("A potential solution could be {:#?}", solution);

    let solution_evaluation = solution.evaluate(&f) * 100 as f32;
    println!("The solution is a {solution_evaluation}% correct");
}
