use clap::Parser;
use problem::{multiples::MultiplesProblem, Problem};
use crate::problem::multiples;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the problem for which you would like a solution as referenced on https://projecteuler.net
    #[arg(short, long)]
    problem: u8
}

pub mod problem;

fn main() {
    let args = Args::parse();

    let mut result: String = String::from("");
    if args.problem == 1 {
        let problem = MultiplesProblem {
            limit: 1000
        };

        result = problem.solve();
    }

    println!("Selected problem: {}", args.problem);
    println!("Solution: {}", result);
}
