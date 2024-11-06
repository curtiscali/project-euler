use super::Problem;
use crate::arithmetic::{linear_sum, quadratic_sum};

pub struct SumSquareDifference {}

impl Problem for SumSquareDifference {
    fn solve(&self) -> String {
        const N: u32 = 100;

        let linear_sum = linear_sum(N);
        let sum_of_squares = quadratic_sum(N);

        return format!("{}", (linear_sum * linear_sum) - sum_of_squares);
    }
}
