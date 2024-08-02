pub mod multiples;
pub mod even_fibonacci;

pub trait Problem {
    fn solve(&self) -> String;
}