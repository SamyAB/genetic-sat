use std::path::PathBuf;

use clap::Parser;

/// SAT solver based on a genetic algorithm
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct InputArguments {
    /// The number of SAT solutions considered each iteration
    #[arg(short, long)]
    pub population_size: i32,

    /// The probability of a solution to randomly change during an iteration
    #[arg(short = 'u', long)]
    pub mutation_probability: f32,

    /// The number of iteration of the algorithm before it stops even if no perfect solution is found
    #[arg(short, long)]
    pub maximum_number_of_generations: i32,

    /// Path to the CNF formula in the dimacs format
    #[arg(short, long)]
    pub formula_path: PathBuf,
}
