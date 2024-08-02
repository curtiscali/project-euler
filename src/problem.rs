pub mod multiples;

pub trait Problem {
    fn solve(&self) -> String;
}