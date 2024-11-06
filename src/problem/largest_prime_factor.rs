use num::integer::Roots;
use crate::primes::primes_below;
use super::Problem;

pub struct LargestPrimeFactorProblem {}

impl Problem for LargestPrimeFactorProblem {
    fn solve(&self) -> String {
        const N: u64 = 600_851_475_143;

        let largest_factor = N.sqrt();
        let primes_under_largest_factor = primes_below::<u64>(largest_factor as usize);
        let mut largest_prime_factor = 2;

        let mut n = primes_under_largest_factor.len() - 1;
        while n > 0 {
            let p = primes_under_largest_factor[n];
            if N % p == 0 && p > largest_prime_factor {
                largest_prime_factor = p;
                break;
            }

            n -= 1; 
        }

        return format!("{}", largest_prime_factor);
    }
}
