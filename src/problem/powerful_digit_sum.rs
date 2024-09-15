use num::BigInt;

use crate::arithmetic::digit_sum;

use super::Problem;

pub struct PowerDigitSumProblem {
    pub upper_bound: usize
}

impl Problem for PowerDigitSumProblem {
    fn solve(&self) -> String {
        let mut max_digit_sum = BigInt::ZERO;

        let mut i = 1;
        while i <= self.upper_bound {
            if i == 1 || i % 10 == 0 {
                i += 1;
                continue;
            }

            let mut j = 1;
            while j <= self.upper_bound {
                let evaluated = BigInt::from(i).pow(j as u32);
                let digit_sum = digit_sum(evaluated.clone());

                if digit_sum > max_digit_sum {
                    max_digit_sum = digit_sum.clone();
                }

                j += 1;
            }

            i += 1;
        }

        return format!("{}", max_digit_sum);
    }
}
