use num::{BigInt, One};
use crate::arithmetic::bigint_digit_sum;
use super::Problem;

// based on the recurrence relation for a(n) defined here:
// https://proofwiki.org/wiki/Continued_Fraction_Expansion_of_Euler%27s_Number
fn a(n: &BigInt) -> BigInt {
    if n % 3 == BigInt::ZERO || n % 3 == BigInt::from(2) {
        return BigInt::one();
    }

    2 * ((n - 1) / 3)
}

pub struct ConvergentsOfEProblem {}

impl Problem for ConvergentsOfEProblem {
    fn solve(&self) -> String {
        // This algorithm based on the following fraction expansion formula from
        // https://proofwiki.org/wiki/Continued_Fraction_Expansion_of_Euler%27s_Number
        // pi = ai * p(i-1) + p(i-2)
        // qi = ai * q(i-1) + q(i-2)
        let mut prev_ps = vec![BigInt::one(), BigInt::one()];

        for i in 2..102 {
            let n = BigInt::from(i);

            let ai = a(&n);
            let new_p = (&ai * &prev_ps[1]) + &prev_ps[0];

            prev_ps[0] = prev_ps[1].clone();
            prev_ps[1] = new_p.clone();
        }
        
        format!("{}", bigint_digit_sum(&prev_ps[1]))
    }
}
