use crate::{combinatorics::partitions_count, primes::primes_below};
use super::Problem;

pub struct PrimeSummationsProblem {}

impl Problem for PrimeSummationsProblem {
    fn name(&self) -> String {
        String::from("Prime Summations")
    }

    fn number(&self) -> u16 {
        77
    }

    fn solve(&self) -> String {
        let mut n = 15u64;
        let mut primes_below_n = primes_below((n - 1) as usize);

        while partitions_count(n, &primes_below_n) <= 5000 {
            n += 1;
            primes_below_n = primes_below((n - 1) as usize);
        }

        format!("{}", n)
    }
}
