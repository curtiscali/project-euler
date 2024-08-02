use std::time::Instant;
use clap::Parser;
use problem::{multiples::MultiplesProblem, Problem, even_fibonacci::EvenFibonacciProblem};

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
    let mut milliseconds: u128 = 0;
    if args.problem == 1 {
        let now = Instant::now();

        let problem = MultiplesProblem {
            limit: 1000
        };

        result = problem.solve();
        milliseconds = now.elapsed().as_millis();
    } else if args.problem == 2 {
        let now = Instant::now();

        let problem = EvenFibonacciProblem {
            limit: 4_000_000i32
        };

        result = problem.solve();
        milliseconds = now.elapsed().as_millis();
    }

    println!("Selected problem: {}", args.problem);
    println!("Solution: {}", result);
    println!("Time taken to solve the problem: {}ms", milliseconds);
}