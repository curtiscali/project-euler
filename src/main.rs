use std::time::Instant;
use std::collections::BTreeMap;
use clap::Parser;
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
    number_splitting::NumberSplittingProblem,
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
    powerful_digit_sum::PowerDigitSumProblem, 
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

fn print_solution(problem_number: u16, problem: &dyn Problem) {
    const NS_TO_US: u128 = 1000;
    const NS_TO_MS: u128 = 1_000_000;
    const NS_TO_S: u128 = 1_000_000_000;

    println!("Selected Problem: {}", problem_number);

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

    let problems_lookup: BTreeMap<u16, Box<dyn Problem>> = BTreeMap::from([
        (1, Box::new(MultiplesProblem {}) as Box<dyn Problem>),
        (2, Box::new(EvenFibonacciProblem {})),
        (3, Box::new(LargestPrimeFactorProblem {})),
        (4, Box::new(LargestPalindromeProduct {})),
        (5, Box::new(SmallestMultipleProblem {})),
        (6, Box::new(SumSquareDifference {}) ),
        (7, Box::new(NthPrimeProblem {})),
        (8, Box::new(LargestProductProblem {})),
        (9, Box::new(SpecialPythagoreanTripletProblem {})),
        (10, Box::new(SummationOfPrimes {})),
        (11, Box::new(LargestProductInAGridProblem {})),
        (12, Box::new(HighlyDivisibleTriangleNumber {})),
        (13, Box::new(LargeSumProblem {})),
        (14, Box::new(LongestCollatzSequenceProblem {})),
        (15, Box::new(LatticePathsProblem {})),
        (16, Box::new(PowerDigitSum {})),
        (17, Box::new(NumberLetterCountsProblem {})),
        (18, Box::new(MaximumPathSumOneProblem {})),
        (19, Box::new(CountingSundaysProblem {})),
        (20, Box::new(FactorialDigitSum {})),
        (21, Box::new(AmicableNumbersProblem {})),
        (22, Box::new(NamesScoresProblem {})),
        (23, Box::new(NonAbundantSumsProblem {})),
        (24, Box::new(LexicographicPermutationsProblem {})),
        (25, Box::new(ThousandDigitFibonacciNumberProblem {})),
        (26, Box::new(ReciprocalCyclesProblem {})),
        (28, Box::new(NumberPowerSpiralsProblem {})),
        (29, Box::new(DistinctPowersProblem {})),
        (30, Box::new(DigitFifthPowersProblem {})),
        (31, Box::new(CoinSumProblem {})),
        (32, Box::new(PandigitalProductsProblem {})),
        (34, Box::new(DigitFactorialsProblem {})),
        (35, Box::new(CircularPrimesProblem {})),
        (36, Box::new(DoubleBasePalindromeProblem {})),
        (38, Box::new(PandigitalMultiplesProblem {})),
        (39, Box::new(IntegerRightTrianglesProblem {})),
        (40, Box::new(ChampernownesConstantProblem {})),
        (41, Box::new(PandigitalPrimeProblem {})),
        (42, Box::new(CodedTriangleNumbersProblem {})),
        (45, Box::new(TriangularPentagonalHexagonalNumberProblem {})),
        (46, Box::new(GoldbachsOtherConjectureProblem {})),
        (47, Box::new(DistinctPrimeFactorsProblem {})),
        (48, Box::new(SelfPowersProblem {})),
        (49, Box::new(PrimePermutationsProblem {})),
        (50, Box::new(ConsecutivePrimeSumProblem {})),
        (52, Box::new(PermutedMultiplesProblem {})),
        (53, Box::new(CombinatoricSelectionsProblem {})),
        (55, Box::new(LychrelNumbersProblem {})),
        (56, Box::new(PowerDigitSumProblem {})),
        (57, Box::new(SquareRootConvergentsProblem {})),
        (58, Box::new(SpiralPrimesProblem {})),
        (63, Box::new(PowerfulDigitCountsProblem {})),
        (64, Box::new(OddPeriodSquareRootsProblem {})),
        (65, Box::new(ConvergentsOfEProblem {})),
        (67, Box::new(MaximumPathSumTwoProblem {})),
        (69, Box::new(TotientMaximumProblem {})),
        (70, Box::new(TotientPermutationProblem {})),
        (71, Box::new(OrderedFractionsProblem {})),
        (72, Box::new(CountingFractionsProblem {})),
        (73, Box::new(CountingFractionsInARangeProblem {})),
        (74, Box::new(DigitFactorialChainsProblem {})),
        (76, Box::new(CountingSummationsProblem {})),
        (77, Box::new(PrimeSummationsProblem {})),
        (78, Box::new(CoinPartitionsProblem {})),
        (79, Box::new(PasscodeDerivationProblem {})),
        (81, Box::new(PathSumTwoWaysProblem {})),
        (82, Box::new(PathSumThreeWaysProblem {})),
        (83, Box::new(PathSumFourWaysProblem {})),
        (85, Box::new(CountingRectanglesProblem {})),
        (89, Box::new(RomanNumeralsProblem {})),
        (92, Box::new(SquareDigitChainsProblem {})),
        (97, Box::new(LargeNonMersennePrimeProblem {})),
        (99, Box::new(LargestExponentialProblem {})),
        (102, Box::new(TriangleContainmentProblem {})),
        (112, Box::new(BouncyNumbersProblem {})),
        (113, Box::new(NonBouncyNumbersProblem {})),
        (122, Box::new(EfficientExponentiationProblem {})),
        (124, Box::new(OrderedRadicalsProblem {})),
        (144, Box::new(LaserBeamReflectionsProblem {})),
        (145, Box::new(ReversibleNumbersProblem {})),
        (179, Box::new(ConsecutivePositiveDivisorsProblem {})),
        (187, Box::new(SemiprimesProblem {})),
        (188, Box::new(HyperexponentiationProblem {})),
        (206, Box::new(ConcealedSquaresProblem {})),
        (401, Box::new(SumOfSquaresOfDivisorsProblem {})),
        (719, Box::new(NumberSplittingProblem {}))
    ]);

    match args.problem {
        Some(problem_number) => {
            match problems_lookup.get(&problem_number) {
                Some(selected_problem) => {
                    print_solution(problem_number, selected_problem.as_ref());
                }
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
