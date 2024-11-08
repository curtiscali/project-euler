use crate::combinatorics::combinations;
use super::Problem;

pub struct LatticePathsProblem {}

impl Problem for LatticePathsProblem {
    fn solve(&self) -> String {
        const GRID_DIM: u128 = 20;
        return format!("{}", combinations(GRID_DIM * 2, GRID_DIM));
    }
}
