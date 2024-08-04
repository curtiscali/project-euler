use std::time::Instant;
use std::collections::BTreeMap;
use clap::Parser;
use problem::{
    multiples::MultiplesProblem, 
    Problem, 
    even_fibonacci::EvenFibonacciProblem, 
    largest_palindrome_product::LargestPalindromeProduct, 
    sum_square_difference::SumSquareDifference,
    prime_10001::NthPrimeProblem, 
    summation_of_primes::SummationOfPrimes, 
    highly_divisible_triangle_number::HighlyDivisibleTriangleNumber,
    large_sum::LargeSumProblem,
    largest_product::LargestProductProblem,
    special_pythagorean_triplet::SpecialPythagoreanTripletProblem
};

pub mod problem;
pub mod primes;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the problem for which you would like a solution as referenced on https://projecteuler.net
    #[arg(short, long)]
    problem: Option<u8>
}

fn print_solution(problem_number: u8, problem: &dyn Problem) {
    println!("Selected Problem: {}", problem_number);

    let mut result: String = String::from("");
    let mut milliseconds: u128 = 0;
    let now = Instant::now();

    result = problem.solve();
    milliseconds = now.elapsed().as_millis();

    println!("Solution: {}", result);
    println!("Time taken to solve the problem: {}ms", milliseconds);
}

fn main() {
    let args = Args::parse();

    let problems_lookup: BTreeMap<u8, Box<dyn Problem>> = BTreeMap::from([
        (1, Box::new(MultiplesProblem { limit: 1000 }) as Box<dyn Problem>),
        (2, Box::new(EvenFibonacciProblem { limit: 4_000_000 }) as Box<dyn Problem>),
        (4, Box::new(LargestPalindromeProduct { limit: 1000 }) as Box<dyn Problem>),
        (6, Box::new(SumSquareDifference { count: 100 }) as Box<dyn Problem>),
        (7, Box::new(NthPrimeProblem { n: 10001 }) as Box<dyn Problem>),
        (8, Box::new(LargestProductProblem {}) as Box<dyn Problem>),
        (9, Box::new(SpecialPythagoreanTripletProblem { target_sum: 1000 }) as Box<dyn Problem>),
        (10, Box::new(SummationOfPrimes { upper_bound: 2_000_000 }) as Box<dyn Problem>),
        (12, Box::new(HighlyDivisibleTriangleNumber { num_divisors: 500 }) as Box<dyn Problem>),
        (13, Box::new(LargeSumProblem {}) as Box<dyn Problem>)
    ]);

    if args.problem.is_some() {
        let problem_number = args.problem.unwrap();
        if problems_lookup.contains_key(&problem_number) {
            let selected_problem = problems_lookup.get(&problem_number).unwrap();
            print_solution(problem_number, selected_problem.as_ref());
        } else {
            println!("Problem {} has not yet been solved", problem_number);
        }
    } else {
        println!("No problem specified. Showing solutions to all solved problems");

        for problem_number in problems_lookup.keys() {
            let selected_problem = problems_lookup.get(&problem_number).unwrap();
            print_solution(*problem_number, selected_problem.as_ref());

            print!("\n\n");
        }
    } 
}
