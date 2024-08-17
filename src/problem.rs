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

pub trait Problem {
    fn solve(&self) -> String;
}