use num::BigInt;
use crate::number_theory::bigint_digit_sum;
use super::Problem;

pub struct PowerfulDigitSumProblem {}

impl Problem for PowerfulDigitSumProblem {
    fn name(&self) -> String {
        String::from("Powerful Digit Sum")
    }

    fn number(&self) -> u16 {
        56
    }

    fn solve(&self) -> String {
        let mut max_digit_sum = BigInt::ZERO;

        for i in 1..=100 {
            if i == 1 || i % 10 == 0 {
                continue;
            }

            for j in 1u32..=100 {
                let evaluated = BigInt::from(i).pow(j);
                let digit_sum = bigint_digit_sum(&evaluated);

                if digit_sum > max_digit_sum {
                    max_digit_sum = digit_sum.clone();
                }
            }
        }

        return format!("{}", max_digit_sum);
    }
}
