use num::BigInt;
use crate::arithmetic::bigint_digit_sum;
use super::Problem;

pub struct PowerDigitSumProblem {}

impl Problem for PowerDigitSumProblem {
    fn solve(&self) -> String {
        let mut max_digit_sum = BigInt::ZERO;

        for i in 1..=100 {
            if i == 1 || i % 10 == 0 {
                continue;
            }

            for j in 1..=100 {
                let evaluated = BigInt::from(i).pow(j as u32);
                let digit_sum = bigint_digit_sum(&evaluated);

                if digit_sum > max_digit_sum {
                    max_digit_sum = digit_sum.clone();
                }
            }
        }

        return format!("{}", max_digit_sum);
    }
}
