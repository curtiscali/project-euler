use num::BigInt;
use crate::number_theory::bigint_digit_sum;
use super::Problem;

fn factorial(n: u32) -> BigInt {
    let mut product = BigInt::from(1);

    for i in 2..=n {
        product = product * i;
    }

    return product;
}

pub struct FactorialDigitSum {}

impl Problem for FactorialDigitSum {
    fn solve(&self) -> String {
        let factorial = factorial(100);

        return format!("{}", bigint_digit_sum(&factorial));
    }
}
