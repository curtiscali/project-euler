use num::BigInt;
use crate::arithmetic::{bigint_fast_pow, bigint_num_digits};
use super::Problem;

pub struct PowerfulDigitCountsProblem {}

impl Problem for PowerfulDigitCountsProblem {
    fn solve(&self) -> String {
        let mut num_powerful_numbers = 0u32;

        for i in 1..=10 {
            for j in 1..=100 {
                let big_i = BigInt::from(i);
                let big_j = BigInt::from(j);

                let exponent = bigint_fast_pow(&big_i, &big_j);
                if &bigint_num_digits(&exponent) == &big_j {
                    num_powerful_numbers += 1;
                }
            }
        }

        format!("{}", num_powerful_numbers)
    }
}