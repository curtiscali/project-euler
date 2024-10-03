use crate::combinatorics::combinations;
use super::Problem;

pub struct LatticePathsProblem {
    pub grid_dim: u128,
}

impl Problem for LatticePathsProblem {
    fn solve(&self) -> String {
        return format!("{}", combinations(self.grid_dim * 2, self.grid_dim));
    }
}
