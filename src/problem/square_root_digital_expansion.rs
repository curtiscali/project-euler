use num::BigInt;
use crate::number_theory::{
    bigint_digit_sum, 
    bigint_fast_pow, 
    bigint_num_digits, 
    is_perfect_square
};
use super::Problem;

// Based on this integer-based algo: http://www.afjarvis.org.uk/maths/jarvisspec02.pdf
fn approx_sqrt(n: &BigInt, precision: &BigInt) -> BigInt {
    let (mut a, mut b) = (BigInt::from(5 * n), BigInt::from(5));

    while &b < precision {
        if &a >= &b {
            a -= &b;
            b += 10;
        } else {
            a *= 100;

            b *= 10;
            b -= 45;
        }
    }

    b
}

pub struct SquareRootDigitalExpansionProblem {}

impl Problem for SquareRootDigitalExpansionProblem {
    fn name(&self) -> String {
        String::from("Square Root Digital Expansion")
    }

    fn number(&self) -> u16 {
        80
    }

    fn solve(&self) -> String {
        // 10 ^ 110 will have at least 110 digits, which is enough precision
        // to make sure the first 100 digits we need are accurate
        let precision: BigInt = BigInt::from(10).pow(110);
        let ten = BigInt::from(10);

        let mut total_digit_sum = BigInt::ZERO;

        for n in 1u32..=100 {
            if !is_perfect_square(n) {
                let sqrt = approx_sqrt(&BigInt::from(n), &precision);
                let num_digits = bigint_num_digits(&sqrt);
                let num_digits_to_truncate = num_digits - 100;

                let first_100_digits = sqrt / bigint_fast_pow(&ten, &num_digits_to_truncate);

                total_digit_sum += bigint_digit_sum(&first_100_digits);
            }
        }

        format!("{}", total_digit_sum)
    }
}
