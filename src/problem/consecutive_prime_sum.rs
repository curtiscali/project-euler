use crate::primes::sieve_of_atkin;

use super::Problem;

pub struct ConsecutivePrimeSumProblem {
    pub limit: usize
}

impl Problem for ConsecutivePrimeSumProblem {
    fn solve(&self) -> String {
        let sieve_results = sieve_of_atkin(self.limit);

        let mut primes_below_bound: Vec<usize> = vec![];
        let mut i = 2;
        while i < sieve_results.len() {
            if sieve_results[i] {
                primes_below_bound.push(i);
            }

            i += 1;
        }

        let mut prime_sum = 0;
        i = 0;
        while prime_sum < self.limit {
            prime_sum += primes_below_bound[i];
            i += 1;
        }

        let max_prime_index = i - 1;

        let mut largest_addend_count = 0;
        let mut largest_prime = 0;
        
        i = 0;
        while i < max_prime_index {
            let mut sum = 0;
            let mut addends_count = 0;
            let mut j = i + 1;
            while j < max_prime_index {
                sum += primes_below_bound[j];
                addends_count += 1;
                j += 1;
            }

            if sieve_results[sum] && sum <= self.limit && addends_count > largest_addend_count {
                largest_prime = sum;
                largest_addend_count = addends_count;
            }

            i += 1;
        }

        format!("{}", largest_prime)
    }
}