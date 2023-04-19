use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;

use std::io::{prelude::*, BufReader};
use std::path::Path;

use crate::sat::{Clause, Formula};

pub fn parse_dimacs_formula_from_file(file_path: &Path) -> Formula {
    let file = File::open(file_path).expect("Could not read formula file");
    let reader = BufReader::new(file);
    let mut clauses = Vec::new();
    let mut all_literals: HashSet<usize> = HashSet::new();

    for line in reader.lines() {
        let literal_values_as_string =
            line.expect("Formula file contains non valid UTF8 character");
        if literal_values_as_string.starts_with('c') || literal_values_as_string.starts_with('p') {
            continue;
        }
        if literal_values_as_string.contains('%') {
            break;
        }

        let clause = parse_clause(&literal_values_as_string);
        let literals_set: HashSet<usize> = clause.literals.keys().cloned().collect();
        all_literals.extend(&literals_set);
        clauses.push(clause);
    }

    let number_of_literals = all_literals.len();
    Formula {
        clauses,
        number_of_literals,
    }
}

fn parse_clause(clause_as_string: &str) -> Clause {
    let mut literals = HashMap::new();

    for literal_value_as_string in clause_as_string.split_whitespace() {
        let literal_value: i32 = literal_value_as_string
            .parse()
            .expect("Formula file contains a literal that is not an interger");
        match literal_value.cmp(&0) {
            Ordering::Less => {
                let literal_key = -literal_value as usize - 1;
                literals.insert(literal_key, false);
            }
            Ordering::Greater => {
                let literal_key = literal_value as usize - 1;
                literals.insert(literal_key, true);
            }
            Ordering::Equal => {
                println!("There is a 0 literal in the dimacs file. It will be ignored");
            }
        }
    }

    Clause { literals }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_clause_offsets_literals_by_minus_1() {
        let clause_as_string_slice = "1 -2 -3";

        let mut expected_literals = HashMap::new();
        expected_literals.insert(0, true);
        expected_literals.insert(1, false);
        expected_literals.insert(2, false);
        let expected_clause = Clause {
            literals: expected_literals,
        };

        let clause = parse_clause(clause_as_string_slice);

        assert_eq!(clause, expected_clause);
    }

    #[test]
    fn test_parse_clause_ignores_0() {
        let clause_as_string_slice = "1 0";

        let mut expected_literals = HashMap::new();
        expected_literals.insert(0, true);
        let expected_clause = Clause {
            literals: expected_literals,
        };

        let clause = parse_clause(clause_as_string_slice);

        assert_eq!(clause, expected_clause);
    }

    #[test]
    fn test_parse_clause_ignores_space_at_the_begining_of_the_clause() {
        let clause_as_string_slice = " 21 4";

        let mut expected_literals = HashMap::new();
        expected_literals.insert(20, true);
        expected_literals.insert(3, true);
        let expected_clause = Clause {
            literals: expected_literals,
        };

        let clause = parse_clause(clause_as_string_slice);

        assert_eq!(clause, expected_clause);
    }

    #[test]
    fn test_parse_dimacs_formula_from_file() {
        let tmp_dir = tempdir().expect("Could not create test temporary directory");
        let dimacs_file_path = tmp_dir.path().join("dimacs.cnf");
        let mut dimacs_file =
            File::create(&dimacs_file_path).expect("Temporary test file could not be created");
        write!(
            dimacs_file,
            "\
c This formula has been created to test cugen
c
c    horn? no
c    forced? no
c    mixed sat? no
c    clause length = 3
c
p cnf 4  5
 1 -2 3 4 0
3 0
-2 0
1 0
4 0
%
0
"
        )
        .expect("Could not write content in dimacs file");

        let mut expected_first_clause_literals = HashMap::new();
        expected_first_clause_literals.insert(0, true);
        expected_first_clause_literals.insert(1, false);
        expected_first_clause_literals.insert(2, true);
        expected_first_clause_literals.insert(3, true);
        let mut expected_second_clause_literals = HashMap::new();
        expected_second_clause_literals.insert(2, true);
        let mut expected_third_clause_literals = HashMap::new();
        expected_third_clause_literals.insert(1, false);
        let mut expected_fourth_clause_literals = HashMap::new();
        expected_fourth_clause_literals.insert(0, true);
        let mut expected_fifth_clause_literals = HashMap::new();
        expected_fifth_clause_literals.insert(3, true);
        let expected_first_clause = Clause {
            literals: expected_first_clause_literals,
        };
        let expected_second_clause = Clause {
            literals: expected_second_clause_literals,
        };
        let expected_third_clause = Clause {
            literals: expected_third_clause_literals,
        };
        let expected_fourth_clause = Clause {
            literals: expected_fourth_clause_literals,
        };
        let expected_fifth_clause = Clause {
            literals: expected_fifth_clause_literals,
        };
        let expected_formula = Formula {
            clauses: vec![
                expected_first_clause,
                expected_second_clause,
                expected_third_clause,
                expected_fourth_clause,
                expected_fifth_clause,
            ],
            number_of_literals: 4,
        };

        let formula = parse_dimacs_formula_from_file(dimacs_file_path.as_path());

        assert_eq!(formula, expected_formula);
    }

    #[test]
    fn parse_dimacs_formula_from_file_should_fill_assign_the_number_of_literals_in_the_formula() {
        let tmp_dir = tempdir().expect("Could not create test temporary directory");
        let dimacs_file_path = tmp_dir.path().join("dimacs.cnf");
        let mut dimacs_file =
            File::create(&dimacs_file_path).expect("Temporary test file could not be created");
        write!(
            dimacs_file,
            "\
c This formula has been created to test cugen
c
c    horn? no
c    forced? no
c    mixed sat? no
c    clause length = 3
c
p cnf 4  5
 1 -2 3 4 0
3 0
-2 0
1 0
4 0
%
0
"
        )
        .expect("Could not write content in dimacs file");

        let expected_number_of_literals = 4;

        let formula = parse_dimacs_formula_from_file(&dimacs_file_path);

        assert_eq!(formula.number_of_literals, expected_number_of_literals);
    }
}
