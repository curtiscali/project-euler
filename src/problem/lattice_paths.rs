use crate::combinatorics::combinations;
use super::Problem;

pub struct LatticePathsProblem { }

impl Problem for LatticePathsProblem {
    fn solve(&self) -> String {
            return format!("{}", combinations(40u64, 20u64));
    }
}
