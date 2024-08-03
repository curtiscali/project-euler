pub mod multiples;
pub mod even_fibonacci;
pub mod largest_palindrome_product;

pub trait Problem {
    fn solve(&self) -> String;
}