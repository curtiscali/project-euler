use crate::number_theory::linear_sum;
use super::Problem;

fn sum_divisible_by_under(target: u32, n: u32) -> u32 {
    let p = target / n;
    return n * linear_sum(p);
}

pub struct MultiplesProblem {}

impl Problem for MultiplesProblem {
    fn solve(&self) -> String {
        const LIMIT: u32 = 1000;

        let sum = sum_divisible_by_under(LIMIT - 1, 3) +
            sum_divisible_by_under(LIMIT - 1, 5) -
            sum_divisible_by_under(LIMIT - 1, 15);

        return format!("{}", sum);
    }
}
