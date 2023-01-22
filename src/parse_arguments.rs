use clap::Parser;

/// SAT solver based on a genetic algorithm
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct InputArguments {
    /// The number of SAT solutions considered each iteration
    #[arg(short, long)]
    population_size: usize,

    /// The probability of a solution to randomly change during an iteration
    #[arg(short = 'u', long)]
    mutation_probability: f32,

    /// The number of iteration of the algorithm before it stops even if no perfect solution is found
    #[arg(short, long)]
    maximum_number_of_generations: Option<usize>,

    /// Path to the CNF formula in the dimacs format
    #[arg(short, long)]
    formula_path: String,
}
