pub mod multiples;
pub mod even_fibonacci;
pub mod largest_palindrome_product;
pub mod sum_square_difference;

pub trait Problem {
    fn solve(&self) -> String;
}