use std::collections::HashMap;

use genetic_sat::{Clause, Formula, Solution};

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

    println!("The formula is {:#?}", f);
    println!("A potential solution could be {:#?}", solution);

    let solution_evaluation = solution.evaluate(&f) * 100 as f32;
    println!("The solution is a {solution_evaluation}% correct");
}
