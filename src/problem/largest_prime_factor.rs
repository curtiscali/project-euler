use crate::{arithmetic::sqrt_usize, primes::sieve_of_atkin};

use super::Problem;

pub struct LargestPrimeFactorProblem {
    pub n: usize
}

impl Problem for LargestPrimeFactorProblem {
    fn solve(&self) -> String {
        let largest_factor = sqrt_usize(self.n);
        let primes_under_largest_factor = sieve_of_atkin(largest_factor);
        let mut largest_prime_factor = 2;

        let mut n = primes_under_largest_factor.len() - 1;
        while n >= 2 {
            if primes_under_largest_factor[n] && self.n % n == 0 {
                largest_prime_factor = n;
                break;
            }

            n -= 1; 
        }

        return format!("{}", largest_prime_factor);
    }
}
