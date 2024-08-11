use std::time::Instant;
use std::collections::BTreeMap;
use clap::Parser;
use problem::{
    distinct_powers::DistinctPowersProblem, 
    even_fibonacci::EvenFibonacciProblem, 
    factorial_digit_sum::FactorialDigitSum, 
    highly_divisible_triangle_number::HighlyDivisibleTriangleNumber, 
    large_sum::LargeSumProblem, 
    largest_exponential::LargestExponentialProblem, 
    largest_palindrome_product::LargestPalindromeProduct, 
    largest_product::LargestProductProblem, 
    lattice_paths::LatticePathsProblem, 
    lexicographic_permutations::LexicographicPermutationsProblem, 
    longest_collatz_sequence::LongestCollatzSequenceProblem, 
    multiples::MultiplesProblem, 
    number_letter_counts::NumberLetterCountsProblem, 
    number_power_spirals::NumberPowerSpiralsProblem, 
    power_digit_sum::PowerDigitSum, 
    prime_10001::NthPrimeProblem, 
    special_pythagorean_triplet::SpecialPythagoreanTripletProblem, 
    sum_square_difference::SumSquareDifference, 
    summation_of_primes::SummationOfPrimes, 
    triangle_containment::TriangleContainmentProblem,
    digit_fifth_powers::DigitFifthPowersProblem,
    digit_factorials::DigitFactorialsProblem,
    coded_triangle_numbers::CodedTriangleNumbersProblem,
    champernownes_constant::ChampernownesConstantProblem,
    pandigital_prime::PandigitalPrimeProblem,
    self_powers::SelfPowersProblem,
    names_scores::NamesScoresProblem,
    Problem
};

pub mod problem;
pub mod primes;
pub mod arithmetic;
pub mod strings;

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

    let now = Instant::now();

    let result = problem.solve();
    let milliseconds = now.elapsed().as_millis();

    println!("Solution: {}", result);
    println!("Time taken to solve the problem: {}ms", milliseconds);
}

fn main() {
    let args = Args::parse();

    let problems_lookup: BTreeMap<u8, Box<dyn Problem>> = BTreeMap::from([
        (1, Box::new(MultiplesProblem { limit: 1000 }) as Box<dyn Problem>),
        (2, Box::new(EvenFibonacciProblem { limit: 4_000_000 })),
        (4, Box::new(LargestPalindromeProduct { limit: 1000 })),
        (6, Box::new(SumSquareDifference { count: 100 }) ),
        (7, Box::new(NthPrimeProblem { n: 10001 })),
        (8, Box::new(LargestProductProblem {})),
        (9, Box::new(SpecialPythagoreanTripletProblem { target_sum: 1000 })),
        (10, Box::new(SummationOfPrimes { upper_bound: 2_000_000 })),
        (12, Box::new(HighlyDivisibleTriangleNumber { num_divisors: 500 })),
        (13, Box::new(LargeSumProblem {})),
        (14, Box::new(LongestCollatzSequenceProblem { limit: 1_000_000 })),
        (15, Box::new(LatticePathsProblem {})),
        (16, Box::new(PowerDigitSum {})),
        (17, Box::new(NumberLetterCountsProblem {})),
        (20, Box::new(FactorialDigitSum { n: 100 })),
        (22, Box::new(NamesScoresProblem {})),
        (24, Box::new(LexicographicPermutationsProblem {})),
        (28, Box::new(NumberPowerSpiralsProblem { spiral_size: 1001 })),
        (29, Box::new(DistinctPowersProblem { upper_bound: 100 })),
        (30, Box::new(DigitFifthPowersProblem {})),
        (34, Box::new(DigitFactorialsProblem {})),
        (40, Box::new(ChampernownesConstantProblem {})),
        (41, Box::new(PandigitalPrimeProblem {})),
        (42, Box::new(CodedTriangleNumbersProblem {})),
        (48, Box::new(SelfPowersProblem { upper_bound: 1000 })),
        (99, Box::new(LargestExponentialProblem {})),
        (102, Box::new(TriangleContainmentProblem {})),
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
