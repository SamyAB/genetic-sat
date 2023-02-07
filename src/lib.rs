mod genetics;
pub mod parse_arguments;
mod parse_dimacs;
mod sat;

use parse_arguments::InputArguments;

pub fn run(args: InputArguments) {
    let formula = parse_dimacs::parse_dimacs_formula_from_file(&args.formula_path);
    let best_solution = genetics::optimize(
        &formula,
        args.population_size,
        args.maximum_number_of_generations,
        args.maximum_of_breeding_individuals_in_a_generation,
        args.mutation_probability,
    );

    let best_fitness = best_solution.evaluate(&formula);
    println!("The best solution is {:?}", best_solution);
    println!("It has a fitness of {best_fitness}");
}
