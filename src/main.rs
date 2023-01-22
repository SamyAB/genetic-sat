use clap::Parser;
use genetic_sat::{self, parse_arguments::InputArguments};

fn main() {
    let input_arguments = InputArguments::parse();
    genetic_sat::run(input_arguments);
}
