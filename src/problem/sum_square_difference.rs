use super::Problem;
use crate::arithmetic::{linear_sum, quadratic_sum};

pub struct SumSquareDifference {
    pub count: u32
}

impl Problem for SumSquareDifference {
    fn solve(&self) -> String {
        // this is derived from squaring the formula n(n + 1)/2
        let square_of_sum = linear_sum(self.count).pow(2);

        // sum of squares = n(n+1)(2n+1)/6
        let sum_of_squares = quadratic_sum(self.count);

        return format!("{}", square_of_sum - sum_of_squares);
    }
}
