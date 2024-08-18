use crate::arithmetic::linear_sum;

use super::Problem;

fn sum_divisible_by_under(target: usize, n: usize) -> usize {
    let p = target / n;
    return n * linear_sum(p);
}

pub struct MultiplesProblem {
    pub limit: usize
}

impl Problem for MultiplesProblem {
    fn solve(&self) -> String {
        let sum = sum_divisible_by_under(self.limit - 1, 3) +
            sum_divisible_by_under(self.limit - 1, 5) -
            sum_divisible_by_under(self.limit - 1, 15);

        return format!("{}", sum);
    }
}