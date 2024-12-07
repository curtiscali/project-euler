use num::{integer::Roots, BigInt};
use crate::number_theory::{bigint_digit_sum, digit_sum, is_perfect_square};
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
        let precision: BigInt = BigInt::from(10).pow(115);

        let mut total_digit_sum = BigInt::ZERO;

        for n in 1u32..=100 {
            if is_perfect_square(n) {
                total_digit_sum += digit_sum(n.sqrt());
            } else {
                let sqrt = approx_sqrt(&BigInt::from(n), &precision);
                let first_100_digits = sqrt / 10u64.pow(15);

                total_digit_sum += bigint_digit_sum(&first_100_digits);
            }
        }

        format!("{}", total_digit_sum)
    }
}
