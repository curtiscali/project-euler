use num::{BigInt, One};
use crate::number_theory::bigint_quadratic_sum;
use super::Problem;

// This function based on an algorithm from: https://www.ivl-projecteuler.com/overview-of-problems/25-difficulty/problem-401
// SIGMA2(n) = sum(f(n/k) + (k^2 * n/k))(k = 1 to sqrt(n)) - sqrt(n) * f(sqrt(n))
// f(n) = (n * (n + 1) * (2n + 1)) / 6
fn sigma2_sum(n: u64) -> BigInt {
    let mut sigma2_sum = BigInt::ZERO;
    let n_sqrt = BigInt::from(n).sqrt();
    let rhs = &n_sqrt * bigint_quadratic_sum(&n_sqrt);

    let mut i = BigInt::one();
    while i <= n_sqrt {
        let ratio = n / &i;
        sigma2_sum += (&i * &i * &ratio) + bigint_quadratic_sum(&ratio);
        i += 1;
    }

    sigma2_sum - rhs
}

pub struct SumOfSquaresOfDivisorsProblem {}

impl Problem for SumOfSquaresOfDivisorsProblem {
    fn name(&self) -> String {
        String::from("Sum of Squares of Divisors")
    }

    fn number(&self) -> u16 {
        401
    }

    fn solve(&self) -> String {
        const N: u64 = 1_000_000_000_000_000;
        const MOD: u64 = 1_000_000_000;

        format!("{}", sigma2_sum(N) % MOD)
    }
}
