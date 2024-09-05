use crate::{arithmetic::sqrt_usize, primes::primes_below};

use super::Problem;

pub struct LargestPrimeFactorProblem {
    pub n: usize
}

impl Problem for LargestPrimeFactorProblem {
    fn solve(&self) -> String {
        let largest_factor = sqrt_usize(self.n);
        let primes_under_largest_factor = primes_below(largest_factor);
        let mut largest_prime_factor = 2;

        let mut i = primes_under_largest_factor.len() - 1;
        while i >= 0 {
            let n = i + 2;

            if primes_under_largest_factor[i] && self.n % n == 0 {
                largest_prime_factor = n;
                break;
            }

            i -= 1; 
        }

        return format!("{}", largest_prime_factor);
    }
}
