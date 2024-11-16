use super::Problem;
use crate::number_theory::{linear_sum, quadratic_sum};

pub struct SumSquareDifference {}

impl Problem for SumSquareDifference {
    fn name(&self) -> String {
        String::from("Sum Square Difference")
    }

    fn number(&self) -> u16 {
        6
    }

    fn solve(&self) -> String {
        const N: u32 = 100;

        let linear_sum = linear_sum(N);
        let sum_of_squares = quadratic_sum(N);

        return format!("{}", (linear_sum * linear_sum) - sum_of_squares);
    }
}
