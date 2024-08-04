pub mod multiples;
pub mod even_fibonacci;
pub mod largest_palindrome_product;
pub mod sum_square_difference;
pub mod prime_10001;
pub mod summation_of_primes;
pub mod highly_divisible_triangle_number;
pub mod large_sum;

pub trait Problem {
    fn solve(&self) -> String;
}