use std::time::Instant;
use std::collections::BTreeMap;
use clap::Parser;
use problem::{
    champernownes_constant::ChampernownesConstantProblem, 
    circular_primes::CircularPrimesProblem, 
    coded_triangle_numbers::CodedTriangleNumbersProblem, 
    digit_factorials::DigitFactorialsProblem, 
    digit_fifth_powers::DigitFifthPowersProblem, 
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
    names_scores::NamesScoresProblem,
    number_letter_counts::NumberLetterCountsProblem, 
    number_power_spirals::NumberPowerSpiralsProblem, 
    pandigital_prime::PandigitalPrimeProblem, 
    power_digit_sum::PowerDigitSum, 
    prime_10001::NthPrimeProblem, 
    self_powers::SelfPowersProblem, 
    special_pythagorean_triplet::SpecialPythagoreanTripletProblem, 
    sum_square_difference::SumSquareDifference, 
    summation_of_primes::SummationOfPrimes, 
    triangle_containment::TriangleContainmentProblem,
    triangular_pentagonal_hexagonal::TriangularPentagonalHexagonalNumberProblem,
    integer_right_triangles::IntegerRightTrianglesProblem,
    permuted_multiples::PermutedMultiplesProblem,
    combinatoric_selections::CombinatoricSelectionsProblem,
    lychrel_numbers::LychrelNumbersProblem,
    powerful_digit_sum::PowerDigitSumProblem,
    totient_maximum::TotientMaximumProblem,
    roman_numerals::RomanNumeralsProblem,
    coin_sums::CoinSumProblem,
    reversible_numbers::ReversibleNumbersProblem,
    counting_rectangles::CountingRectanglesProblem,
    square_digit_chains::SquareDigitChainsProblem,
    laser_beam_reflections::LaserBeamReflectionsProblem,
    large_non_mersenne_prime::LargeNonMersennePrimeProblem,
    double_base_palindrome::DoubleBasePalindromeProblem,
    Problem
};

pub mod problem;
pub mod primes;
pub mod arithmetic;
pub mod strings;
pub mod combinatorics;
pub mod linear_algebra;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the problem for which you would like a solution as referenced on https://projecteuler.net
    #[arg(short, long)]
    problem: Option<u16>
}

fn print_solution(problem_number: u16, problem: &dyn Problem) {
    println!("Selected Problem: {}", problem_number);

    let now = Instant::now();

    let result = problem.solve();
    let milliseconds = now.elapsed().as_millis();

    println!("Solution: {}", result);
    println!("Time taken to solve the problem: {}ms", milliseconds);
}

fn main() {
    let args = Args::parse();

    let problems_lookup: BTreeMap<u16, Box<dyn Problem>> = BTreeMap::from([
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
        (31, Box::new(CoinSumProblem {
            total_pence: 200,
            denominations: vec![1, 2, 5, 10, 20, 50, 100, 200]
        })),
        (34, Box::new(DigitFactorialsProblem {})),
        (35, Box::new(CircularPrimesProblem { upper_bound: 1_000_000 })),
        (36, Box::new(DoubleBasePalindromeProblem { upper_bound: 1_000_000 })),
        (39, Box::new(IntegerRightTrianglesProblem { max_perimeter: 1000 })),
        (40, Box::new(ChampernownesConstantProblem {})),
        (41, Box::new(PandigitalPrimeProblem {})),
        (42, Box::new(CodedTriangleNumbersProblem {})),
        (45, Box::new(TriangularPentagonalHexagonalNumberProblem {})),
        (48, Box::new(SelfPowersProblem { upper_bound: 1000 })),
        (52, Box::new(PermutedMultiplesProblem {})),
        (53, Box::new(CombinatoricSelectionsProblem { upper_bound: 100, combination_limit: 1_000_000 })),
        (55, Box::new(LychrelNumbersProblem { upper_bound: 10_000 })),
        (56, Box::new(PowerDigitSumProblem { upper_bound: 100 })),
        (69, Box::new(TotientMaximumProblem { upper_bound: 1_000_000 })),
        (85, Box::new(CountingRectanglesProblem { target_rectangle_count: 2_000_000 })),
        (89, Box::new(RomanNumeralsProblem {})),
        (92, Box::new(SquareDigitChainsProblem { upper_bound: 10_000_000})),
        (97, Box::new(LargeNonMersennePrimeProblem {})),
        (99, Box::new(LargestExponentialProblem {})),
        (102, Box::new(TriangleContainmentProblem {})),
        (144, Box::new(LaserBeamReflectionsProblem {})),
        (145, Box::new(ReversibleNumbersProblem { upper_bound: 1_000_000_000 }))
    ]);

    match args.problem {
        Some(problem_number) => {
            match problems_lookup.get(&problem_number) {
                Some(selected_problem) => {
                    print_solution(problem_number, selected_problem.as_ref());
                },
                None => {
                    println!("Problem {} has not yet been solved", problem_number);
                }
            }
        },
        None => {
            println!("No problem specified. Showing solutions to all solved problems");

            for problem_number in problems_lookup.keys() {
                let selected_problem = problems_lookup.get(&problem_number).unwrap();
                print_solution(*problem_number, selected_problem.as_ref());
    
                print!("\n\n");
            }
        }
    }
}
