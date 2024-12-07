use num::BigInt;
use crate::number_theory::{bigint_num_digits, is_perfect_square};
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

        for n in 1u32..=100 {

        }

        format!("")
    }
}
