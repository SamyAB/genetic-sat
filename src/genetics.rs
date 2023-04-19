use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use crate::sat::{Formula, Solution};

struct Population {
    individuals: Vec<Solution>, // TODO: replace with something that impl Individual trait
    best_fitness: f32,
}

impl Population {
    fn new(individuals: Vec<Solution>) -> Self {
        Population {
            individuals,
            best_fitness: 0f32,
        }
    }

    fn evaluate(&mut self, formula: &Formula) -> Vec<f32> {
        let population_fitness = self
            .individuals
            .iter()
            .map(|solution| solution.evaluate(formula))
            .collect::<Vec<_>>();
        self.best_fitness = Population::best_fitness(&population_fitness);
        population_fitness.to_vec()
    }

    fn generate_random_individuals(
        individual_size: usize,
        number_of_individuals: i32,
    ) -> Vec<Solution> {
        (0..number_of_individuals)
            .map(|_| {
                let literals: Vec<bool> = (0..individual_size)
                    .map(|_| rand::random::<bool>())
                    .collect();
                Solution { literals }
            })
            .collect()
    }

    fn genesis(individual_size: usize, population_size: i32) -> Self {
        let individuals = Population::generate_random_individuals(individual_size, population_size);
        Population::new(individuals)
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

    fn best_fitness(population_fitness: &[f32]) -> f32 {
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

    fn map_fitness_to_individuals(&self, population_fitness: &[f32]) -> Vec<(Solution, f32)> {
        let mut individual_fitness_map: Vec<(Solution, f32)> = Vec::new();
        for (individual_index, individual) in self.individuals.iter().enumerate() {
            let individual_fitness = *population_fitness.get(individual_index)
                .expect("The population fitness does not have the same number of elements as the number of individuals");
            if individual_fitness > 0. {
                individual_fitness_map.push((individual.clone(), individual_fitness));
            }
        }

        individual_fitness_map
    }

    fn select_breeding_population(
        indivudial_fitness_map: &Vec<(Solution, f32)>,
        number_of_breeding_individuals: i32,
    ) -> Vec<Solution> {
        let mut rng = thread_rng();
        indivudial_fitness_map
            .choose_multiple_weighted(&mut rng, number_of_breeding_individuals as usize, |item| {
                item.1
            })
            .unwrap()
            .map(|item| item.0.clone())
            .collect()
    }

    fn flip_random_literal(mut individual: Solution) -> Solution {
        let mut rng = rand::thread_rng();
        let literal_index_to_flip = (0..individual.literals.len())
            .choose(&mut rng)
            .expect("An individual should have at least a single literal");
        let literal_value_to_flip = individual.literals.remove(literal_index_to_flip);
        individual
            .literals
            .insert(literal_index_to_flip, !literal_value_to_flip);
        individual
    }

    fn coupling(couples: Vec<(&Solution, &Solution)>) -> Vec<Solution> {
        let mut embrios: Vec<Solution> = Vec::new();
        for (first_parent, second_parent) in couples {
            let embrio_literals: Vec<bool> = first_parent
                .literals
                .iter()
                .zip(second_parent.literals.iter())
                .map(|(first_parent_literal, second_parent_literal)| {
                    if rand::random() {
                        *first_parent_literal
                    } else {
                        *second_parent_literal
                    }
                })
                .collect();
            embrios.push(Solution {
                literals: embrio_literals,
            });
        }
        embrios
    }

    fn binary_crossover(
        mut breeding_individuals: Vec<Solution>,
        number_of_individuals: i32,
    ) -> Vec<Solution> {
        let mut embrios: Vec<Solution> = Vec::new();
        let mut rng = rand::thread_rng();
        loop {
            if embrios.len() >= number_of_individuals as usize {
                break;
            }
            breeding_individuals.shuffle(&mut rng);
            let couples: Vec<(&Solution, &Solution)> = breeding_individuals
                .iter()
                .enumerate()
                .filter(|&(index, _)| index % 2 == 0)
                .map(|(_, x)| x)
                .zip(
                    breeding_individuals
                        .iter()
                        .enumerate()
                        .filter(|&(index, _)| index % 2 == 1)
                        .map(|(_, x)| x),
                )
                .collect();
            embrios.append(&mut Population::coupling(couples));
        }
        breeding_individuals
    }

    fn mutation(embrios: Vec<Solution>, mutation_probability: f32) -> Vec<Solution> {
        let mut children: Vec<Solution> = Vec::new();

        for embrio in embrios.into_iter() {
            let mutate: bool = rand::random::<f32>() > mutation_probability;
            if mutate {
                let mutated_child = Population::flip_random_literal(embrio);
                children.push(mutated_child);
            } else {
                children.push(embrio);
            }
        }
        children
    }

    fn next_generation(
        &self,
        population_fitness: &[f32],
        maximum_number_of_breeding_individuals: i32,
        number_of_individuals_in_generation: i32,
        mutation_probability: f32,
    ) -> Population {
        let individual_fitness_map = self.map_fitness_to_individuals(population_fitness);
        if individual_fitness_map.len() <= 1 {
            println!("The whole population died. Restarting from scratch");
            return Population::genesis(
                self.individuals
                    .get(0)
                    .expect("A generation should always contain individuals")
                    .literals
                    .len(),
                number_of_individuals_in_generation,
            );
        }

        let breeding_population = Population::select_breeding_population(
            &individual_fitness_map,
            maximum_number_of_breeding_individuals,
        );
        let embrios =
            Population::binary_crossover(breeding_population, number_of_individuals_in_generation);
        let next_gen_individuals = Population::mutation(embrios, mutation_probability);

        Population::new(next_gen_individuals)
    }
}

pub fn optimize(
    formula: &Formula,
    population_size: i32,
    maximum_number_of_generations: i32,
    maximum_number_of_breeding_individuals: i32,
    mutation_probability: f32,
) -> Solution {
    let mut population = Population::genesis(formula.number_of_literals, population_size);
    let mut all_time_best_fitness = 0f32;
    let mut all_time_best_individual = population
        .individuals
        .get(0)
        .expect("Initial population should at least have a single individual")
        .clone();

    for generation in 1..maximum_number_of_generations {
        let population_fitness = population.evaluate(formula);
        let generation_best_fitness = population.best_fitness;
        if generation_best_fitness == 1f32 {
            println!("Prefect individual found!");
            all_time_best_individual = population.best_individual(formula);
            break;
        }
        if generation_best_fitness > all_time_best_fitness {
            println!(
                "During generation {} new all time best fitness has been found: {}",
                generation, generation_best_fitness
            );
            all_time_best_fitness = generation_best_fitness;
            all_time_best_individual = population.best_individual(formula);
        } else if generation % 100 == 0 {
            println!(
                "The generation {generation}. All time best fitness {}",
                all_time_best_fitness,
            );
        }

        population = population.next_generation(
            &population_fitness,
            maximum_number_of_breeding_individuals,
            population_size,
            mutation_probability,
        );
    }

    all_time_best_individual
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_fitness_to_individual_creates_a_map_of_each_individual_and_its_fitness() {
        let indivudials = vec![
            Solution {
                literals: vec![true, false],
            },
            Solution {
                literals: vec![false, true],
            },
        ];
        let population = Population::new(indivudials);
        let population_fitness = vec![0.1, 0.9];

        let expected_individual_fitness_map = vec![
            (
                Solution {
                    literals: vec![true, false],
                },
                0.1,
            ),
            (
                Solution {
                    literals: vec![false, true],
                },
                0.9,
            ),
        ];

        let individual_fitness_map = population.map_fitness_to_individuals(&population_fitness);

        assert_eq!(
            individual_fitness_map.len(),
            expected_individual_fitness_map.len()
        );
        for tuple_index in 0..individual_fitness_map.len() {
            assert_eq!(
                individual_fitness_map.get(tuple_index).unwrap().0,
                expected_individual_fitness_map.get(tuple_index).unwrap().0
            );
            assert_eq!(
                individual_fitness_map.get(tuple_index).unwrap().1,
                expected_individual_fitness_map.get(tuple_index).unwrap().1
            );
        }
    }

    #[test]
    fn test_map_fitness_to_individual_should_drop_indivudials_with_0_fitness() {
        let individuals = vec![
            Solution {
                literals: vec![true, false],
            },
            Solution {
                literals: vec![false, true],
            },
        ];
        let population = Population::new(individuals);
        let population_fitness = vec![0., 0.5];

        let expected_individual_fitness_map = vec![(
            Solution {
                literals: vec![false, true],
            },
            0.5,
        )];

        let individual_fitness_map = population.map_fitness_to_individuals(&population_fitness);

        assert_eq!(
            individual_fitness_map.len(),
            expected_individual_fitness_map.len()
        );
    }

    #[test]
    fn test_choose_individuals_to_breed_returns_the_whole_input_population_when_number_of_bredding_indivudials_equals_population_size(
    ) {
        let individuals = vec![
            Solution {
                literals: vec![true, false],
            },
            Solution {
                literals: vec![false, true],
            },
        ];
        let individual_fitness_map =
            vec![(individuals[0].clone(), 0.5), (individuals[1].clone(), 0.5)];
        let number_of_breeding_individuals = 2;

        let breeding_population = Population::select_breeding_population(
            &individual_fitness_map,
            number_of_breeding_individuals,
        );

        assert!(
            breeding_population[0] == individuals[0] || breeding_population[0] == individuals[1]
        );
        assert!(
            breeding_population[1] == individuals[0] || breeding_population[1] == individuals[1]
        );
    }

    #[test]
    fn test_choose_individuals_to_breed_returns_a_vector_of_individuals_with_a_len_equal_to_number_of_breeding_individual(
    ) {
        let individual_fitness_map = vec![
            (
                Solution {
                    literals: vec![true, false],
                },
                0.5,
            ),
            (
                Solution {
                    literals: vec![false, true],
                },
                0.5,
            ),
            (
                Solution {
                    literals: vec![true, true],
                },
                0.4,
            ),
        ];
        let number_of_breeding_individuals = 2;

        let breeding_population = Population::select_breeding_population(
            &individual_fitness_map,
            number_of_breeding_individuals,
        );

        assert_eq!(
            breeding_population.len(),
            number_of_breeding_individuals as usize
        );
    }

    #[test]
    fn test_choose_individuals_to_breed_returns_an_empty_vector_when_individual_fitness_map_is_empty(
    ) {
        let individual_fitness_map: Vec<(Solution, f32)> = Vec::new();
        let number_of_breeding_indivuduals = 1;

        let breeding_population = Population::select_breeding_population(
            &individual_fitness_map,
            number_of_breeding_indivuduals,
        );

        assert_eq!(breeding_population.len(), 0);
    }

    #[test]
    fn test_choose_individuals_to_breed_returns_an_individual_vector_with_the_same_length_as_individual_fitness_map_when_it_is_smaller_than_number_of_breeding_population(
    ) {
        let individual_fitness_map = vec![(
            Solution {
                literals: vec![true, false],
            },
            0.5,
        )];
        let number_of_breeding_individuals = 2;

        let breeding_population = Population::select_breeding_population(
            &individual_fitness_map,
            number_of_breeding_individuals,
        );

        assert_eq!(breeding_population.len(), individual_fitness_map.len());
    }
}
