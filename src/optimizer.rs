use crate::{Formula, Solution};

struct Population {
    individuals: Vec<Solution>,
}

impl Population {
    fn build(individuals: Vec<Solution>) -> Self {
        Population { individuals }
    }

    fn evaluate(&self, formula: &Formula) -> Vec<f32> {
        self.individuals
            .iter()
            .map(|solution| solution.evaluate(formula))
            .collect()
    }

    fn genesis(indivudal_size: usize, population_size: i32) -> Self {
        let solutions: Vec<Solution> = (0..population_size)
            .map(|_| {
                let literals: Vec<bool> = (0..indivudal_size)
                    .map(|_| rand::random::<bool>())
                    .collect();
                Solution { literals }
            })
            .collect();
        Population::build(solutions)
    }

    fn best_individual(&self, formula: &Formula) -> Solution {
        let mut best_fitness = 0.;
        let mut best_individual_index = 0;

        for (solution_index, solution) in self.individuals.iter().enumerate() {
            let individual_fitness = solution.evaluate(formula);
            if solution.evaluate(formula) > best_fitness {
                best_fitness = individual_fitness;
                best_individual_index = solution_index;
            }
        }

        self.individuals
            .get(best_individual_index)
            .expect("The index has been found looping over the population this should not fail")
            .clone()
    }
}

fn best_fitness(population_fitness: &Vec<f32>) -> f32 {
    population_fitness
        .iter()
        .cloned()
        .fold(0., |current_max, value| {
            if current_max >= value {
                current_max
            } else {
                value
            }
        })
}

pub fn optimize(
    formula: &Formula,
    population_size: i32,
    maximum_number_of_generations: i32,
) -> Solution {
    let population = Population::genesis(formula.number_of_literals, population_size);
    let mut population_fitness = Vec::new();

    for _ in 1..maximum_number_of_generations {
        population_fitness = population.evaluate(formula);
    }

    let population_best_fitness = best_fitness(&population_fitness);
    println!("The best fitness is {}", population_best_fitness);

    population.best_individual(formula)
}
