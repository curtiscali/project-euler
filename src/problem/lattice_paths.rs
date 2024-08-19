use crate::combinatorics::combinations_usize;
use super::Problem;

pub struct LatticePathsProblem { }

impl Problem for LatticePathsProblem {
    fn solve(&self) -> String {
        return format!("{}", combinations_usize(40, 20));
    }
}