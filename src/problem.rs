use std::fmt::Display;

pub mod multiples;
pub mod even_fibonacci;
pub mod largest_palindrome_product;
pub mod sum_square_difference;
pub mod prime_10001;
pub mod summation_of_primes;
pub mod highly_divisible_triangle_number;
pub mod large_sum;
pub mod largest_product;
pub mod special_pythagorean_triplet;
pub mod longest_collatz_sequence;
pub mod lattice_paths;
pub mod power_digit_sum;
pub mod number_letter_counts;
pub mod factorial_digit_sum;
pub mod triangle_containment;
pub mod largest_exponential;
pub mod lexicographic_permutations;
pub mod distinct_powers;
pub mod number_power_spirals;
pub mod digit_fifth_powers;
pub mod digit_factorials;
pub mod coded_triangle_numbers;
pub mod champernownes_constant;
pub mod pandigital_prime;
pub mod self_powers;
pub mod names_scores;
pub mod circular_primes;
pub mod triangular_pentagonal_hexagonal;
pub mod integer_right_triangles;
pub mod permuted_multiples;
pub mod combinatoric_selections;
pub mod lychrel_numbers;
pub mod powerful_digit_sum;
pub mod totient_maximum;
pub mod roman_numerals;
pub mod coin_sums;
pub mod counting_rectangles;
pub mod reversible_numbers;
pub mod square_digit_chains;
pub mod laser_beam_reflections;
pub mod large_non_mersenne_prime;
pub mod double_base_palindrome;
pub mod smallest_multiple;
pub mod largest_prime_factor;
pub mod thousand_digit_fibonacci_number;
pub mod amicable_numbers;
pub mod totient_permutation;
pub mod ordered_fractions;
pub mod counting_fractions_in_a_range;
pub mod counting_fractions;
pub mod prime_permutations;
pub mod sum_of_squares_of_divisors;
pub mod bouncy_numbers;
pub mod hyperexponentiation;
pub mod passcode_derivation;
pub mod consecutive_prime_sum;
pub mod distinct_prime_factors;
pub mod goldbachs_other_conjecture;
pub mod largest_product_in_a_grid;
pub mod non_abundant_sums;
pub mod reciprocal_cycles;
pub mod semiprimes;
pub mod consecutive_positive_divisors;
pub mod maximum_path_sum_one;
pub mod maximum_path_sum_two;
pub mod spiral_primes;
pub mod counting_sundays;
pub mod square_root_convergents;
pub mod convergents_of_e;
pub mod efficient_exponentiation;
pub mod pandigital_products;
pub mod pandigital_multiples;
pub mod ordered_radicals;
pub mod powerful_digit_counts;
pub mod odd_period_square_roots;
pub mod counting_summations;
pub mod prime_summations;
pub mod coin_partitions;
pub mod path_sum_two_ways;
pub mod non_bouncy_numbers;
pub mod path_sum_three_ways;
pub mod path_sum_four_ways;
pub mod digit_factorial_chains;
pub mod concealed_squares;
pub mod problem_500;
pub mod heegner;
pub mod square_root_digital_expansion;

pub trait Problem {
    fn name(&self) -> String;
    fn number(&self) -> u16;
    fn solve(&self) -> String;
}

impl Display for dyn Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Problem {}: {}", self.number(), self.name())
    }
}

pub trait BonusProblem {
    fn name(&self) -> String;
    fn solve(&self) -> String;
}

impl Display for dyn BonusProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Problem {}", self.name())
    }
}
