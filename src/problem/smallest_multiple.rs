use crate::arithmetic::lcm_usize;

use super::Problem;

pub struct SmallestMultipleProblem {}

impl Problem for SmallestMultipleProblem {
    fn solve(&self) -> String {
        let values: Vec<usize> = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        return format!("{}", lcm_usize(&values));
    }
}
