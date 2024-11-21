use std::time::Instant;
use clap::Parser;
use inquire::Select;
use problem::{
    amicable_numbers::AmicableNumbersProblem,
    bouncy_numbers::BouncyNumbersProblem, 
    champernownes_constant::ChampernownesConstantProblem, 
    circular_primes::CircularPrimesProblem, 
    coded_triangle_numbers::CodedTriangleNumbersProblem, 
    coin_partitions::CoinPartitionsProblem,
    coin_sums::CoinSumProblem,
    combinatoric_selections::CombinatoricSelectionsProblem, 
    concealed_squares::ConcealedSquaresProblem,
    consecutive_positive_divisors::ConsecutivePositiveDivisorsProblem, 
    consecutive_prime_sum::ConsecutivePrimeSumProblem, 
    convergents_of_e::ConvergentsOfEProblem, 
    counting_fractions::CountingFractionsProblem, 
    counting_fractions_in_a_range::CountingFractionsInARangeProblem, 
    counting_rectangles::CountingRectanglesProblem, 
    counting_summations::CountingSummationsProblem, 
    counting_sundays::CountingSundaysProblem, 
    digit_factorials::DigitFactorialsProblem, 
    digit_factorial_chains::DigitFactorialChainsProblem,
    digit_fifth_powers::DigitFifthPowersProblem, 
    distinct_powers::DistinctPowersProblem, 
    distinct_prime_factors::DistinctPrimeFactorsProblem, 
    double_base_palindrome::DoubleBasePalindromeProblem, 
    efficient_exponentiation::EfficientExponentiationProblem, 
    even_fibonacci::EvenFibonacciProblem, 
    factorial_digit_sum::FactorialDigitSum, 
    goldbachs_other_conjecture::GoldbachsOtherConjectureProblem, 
    highly_divisible_triangle_number::HighlyDivisibleTriangleNumber, 
    hyperexponentiation::HyperexponentiationProblem, 
    integer_right_triangles::IntegerRightTrianglesProblem, 
    large_non_mersenne_prime::LargeNonMersennePrimeProblem, 
    large_sum::LargeSumProblem, 
    largest_exponential::LargestExponentialProblem, 
    largest_palindrome_product::LargestPalindromeProduct, 
    largest_prime_factor::LargestPrimeFactorProblem, 
    largest_product::LargestProductProblem, 
    largest_product_in_a_grid::LargestProductInAGridProblem, 
    laser_beam_reflections::LaserBeamReflectionsProblem, 
    lattice_paths::LatticePathsProblem, 
    lexicographic_permutations::LexicographicPermutationsProblem, 
    longest_collatz_sequence::LongestCollatzSequenceProblem, 
    lychrel_numbers::LychrelNumbersProblem, 
    maximum_path_sum_one::MaximumPathSumOneProblem, 
    maximum_path_sum_two::MaximumPathSumTwoProblem, 
    multiples::MultiplesProblem, 
    names_scores::NamesScoresProblem, 
    non_abundant_sums::NonAbundantSumsProblem,
    non_bouncy_numbers::NonBouncyNumbersProblem,
    number_letter_counts::NumberLetterCountsProblem, 
    number_power_spirals::NumberPowerSpiralsProblem,
    odd_period_square_roots::OddPeriodSquareRootsProblem, 
    ordered_fractions::OrderedFractionsProblem, 
    ordered_radicals::OrderedRadicalsProblem, 
    pandigital_multiples::PandigitalMultiplesProblem, 
    pandigital_prime::PandigitalPrimeProblem, 
    pandigital_products::PandigitalProductsProblem, 
    passcode_derivation::PasscodeDerivationProblem, 
    path_sum_two_ways::PathSumTwoWaysProblem,
    path_sum_three_ways::PathSumThreeWaysProblem,
    path_sum_four_ways::PathSumFourWaysProblem,
    permuted_multiples::PermutedMultiplesProblem,
    power_digit_sum::PowerDigitSum, 
    powerful_digit_counts::PowerfulDigitCountsProblem, 
    powerful_digit_sum::PowerfulDigitSumProblem, 
    prime_10001::NthPrimeProblem, 
    prime_permutations::PrimePermutationsProblem,
    prime_summations::PrimeSummationsProblem,
    reciprocal_cycles::ReciprocalCyclesProblem, 
    reversible_numbers::ReversibleNumbersProblem, 
    roman_numerals::RomanNumeralsProblem, 
    self_powers::SelfPowersProblem, 
    semiprimes::SemiprimesProblem, 
    smallest_multiple::SmallestMultipleProblem, 
    special_pythagorean_triplet::SpecialPythagoreanTripletProblem, 
    spiral_primes::SpiralPrimesProblem, 
    square_digit_chains::SquareDigitChainsProblem, 
    square_root_convergents::SquareRootConvergentsProblem, 
    sum_of_squares_of_divisors::SumOfSquaresOfDivisorsProblem, 
    sum_square_difference::SumSquareDifference, 
    summation_of_primes::SummationOfPrimes, 
    thousand_digit_fibonacci_number::ThousandDigitFibonacciNumberProblem, 
    totient_maximum::TotientMaximumProblem, 
    totient_permutation::TotientPermutationProblem, 
    triangle_containment::TriangleContainmentProblem, 
    triangular_pentagonal_hexagonal::TriangularPentagonalHexagonalNumberProblem,
    Problem
};

pub mod problem;
pub mod primes;
pub mod number_theory;
pub mod strings;
pub mod combinatorics;
pub mod linear_algebra;
pub mod fractions;
pub mod hyperops;
pub mod triangles;

/// Simple program to greet a person:104
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the problem for which you would like a solution as referenced on https://projecteuler.net
    #[arg(short, long)]
    problem: Option<u16>
}

fn print_solution(problem: &dyn Problem) {
    const NS_TO_US: u128 = 1000;
    const NS_TO_MS: u128 = 1_000_000;
    const NS_TO_S: u128 = 1_000_000_000;

    println!("Selected Problem: {}", problem.number());

    let now = Instant::now();

    let result = problem.solve();
    let nanoseconds = now.elapsed().as_nanos();

    let time_string = if nanoseconds < NS_TO_US {
        format!("{}ns", nanoseconds)
    } else if nanoseconds < NS_TO_MS {
        format!("{}μs", nanoseconds / NS_TO_US)
    } else if nanoseconds < NS_TO_S {
        format!("{}ms", nanoseconds / NS_TO_MS)
    } else {
        format!("{:.3}s", (nanoseconds as f64) / (NS_TO_S as f64))
    };

    println!("Solution: {}", result);
    println!("Time taken to solve the problem: {}", time_string);
}

fn main() {
    let args = Args::parse();

    let solved_problems = vec![
        Box::new(MultiplesProblem {}) as Box<dyn Problem>,
        Box::new(EvenFibonacciProblem {}),
        Box::new(LargestPrimeFactorProblem {}),
        Box::new(LargestPalindromeProduct {}),
        Box::new(SmallestMultipleProblem {}),
        Box::new(SumSquareDifference {}) ,
        Box::new(NthPrimeProblem {}),
        Box::new(LargestProductProblem {}),
        Box::new(SpecialPythagoreanTripletProblem {}),
        Box::new(SummationOfPrimes {}),
        Box::new(LargestProductInAGridProblem {}),
        Box::new(HighlyDivisibleTriangleNumber {}),
        Box::new(LargeSumProblem {}),
        Box::new(LongestCollatzSequenceProblem {}),
        Box::new(LatticePathsProblem {}),
        Box::new(PowerDigitSum {}),
        Box::new(NumberLetterCountsProblem {}),
        Box::new(MaximumPathSumOneProblem {}),
        Box::new(CountingSundaysProblem {}),
        Box::new(FactorialDigitSum {}),
        Box::new(AmicableNumbersProblem {}),
        Box::new(NamesScoresProblem {}),
        Box::new(NonAbundantSumsProblem {}),
        Box::new(LexicographicPermutationsProblem {}),
        Box::new(ThousandDigitFibonacciNumberProblem {}),
        Box::new(ReciprocalCyclesProblem {}),
        Box::new(NumberPowerSpiralsProblem {}),
        Box::new(DistinctPowersProblem {}),
        Box::new(DigitFifthPowersProblem {}),
        Box::new(CoinSumProblem {}),
        Box::new(PandigitalProductsProblem {}),
        Box::new(DigitFactorialsProblem {}),
        Box::new(CircularPrimesProblem {}),
        Box::new(DoubleBasePalindromeProblem {}),
        Box::new(PandigitalMultiplesProblem {}),
        Box::new(IntegerRightTrianglesProblem {}),
        Box::new(ChampernownesConstantProblem {}),
        Box::new(PandigitalPrimeProblem {}),
        Box::new(CodedTriangleNumbersProblem {}),
        Box::new(TriangularPentagonalHexagonalNumberProblem {}),
        Box::new(GoldbachsOtherConjectureProblem {}),
        Box::new(DistinctPrimeFactorsProblem {}),
        Box::new(SelfPowersProblem {}),
        Box::new(PrimePermutationsProblem {}),
        Box::new(ConsecutivePrimeSumProblem {}),
        Box::new(PermutedMultiplesProblem {}),
        Box::new(CombinatoricSelectionsProblem {}),
        Box::new(LychrelNumbersProblem {}),
        Box::new(PowerfulDigitSumProblem {}),
        Box::new(SquareRootConvergentsProblem {}),
        Box::new(SpiralPrimesProblem {}),
        Box::new(PowerfulDigitCountsProblem {}),
        Box::new(OddPeriodSquareRootsProblem {}),
        Box::new(ConvergentsOfEProblem {}),
        Box::new(MaximumPathSumTwoProblem {}),
        Box::new(TotientMaximumProblem {}),
        Box::new(TotientPermutationProblem {}),
        Box::new(OrderedFractionsProblem {}),
        Box::new(CountingFractionsProblem {}),
        Box::new(CountingFractionsInARangeProblem {}),
        Box::new(DigitFactorialChainsProblem {}),
        Box::new(CountingSummationsProblem {}),
        Box::new(PrimeSummationsProblem {}),
        Box::new(CoinPartitionsProblem {}),
        Box::new(PasscodeDerivationProblem {}),
        Box::new(PathSumTwoWaysProblem {}),
        Box::new(PathSumThreeWaysProblem {}),
        Box::new(PathSumFourWaysProblem {}),
        Box::new(CountingRectanglesProblem {}),
        Box::new(RomanNumeralsProblem {}),
        Box::new(SquareDigitChainsProblem {}),
        Box::new(LargeNonMersennePrimeProblem {}),
        Box::new(LargestExponentialProblem {}),
        Box::new(TriangleContainmentProblem {}),
        Box::new(BouncyNumbersProblem {}),
        Box::new(NonBouncyNumbersProblem {}),
        Box::new(EfficientExponentiationProblem {}),
        Box::new(OrderedRadicalsProblem {}),
        Box::new(LaserBeamReflectionsProblem {}),
        Box::new(ReversibleNumbersProblem {}),
        Box::new(ConsecutivePositiveDivisorsProblem {}),
        Box::new(SemiprimesProblem {}),
        Box::new(HyperexponentiationProblem {}),
        Box::new(ConcealedSquaresProblem {}),
        Box::new(SumOfSquaresOfDivisorsProblem {}),
    ];


    match args.problem {
        Some(problem_number) => {
            match solved_problems.binary_search_by(|p| p.number().cmp(&problem_number)) {
                Ok(idx_of_problem) => {
                    let selected_problem = &solved_problems[idx_of_problem];
                    print_solution(selected_problem.as_ref());
                }, 
                Err(_) => {
                    println!("Problem {} has not yet been solved", problem_number);
                }
            }
        },
        None => {
            let selection = Select::new(
                "Select the problem for which you'd like to see the solution from the list below", 
                solved_problems
            ).with_vim_mode(true).prompt();

            match selection {
                Ok(selected_problem) => {
                    print_solution(selected_problem.as_ref());
                },
                Err(_) => println!("Must select a problem.")
            }
        }
    }
}
