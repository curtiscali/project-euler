use crate::{primes::primes_below, Problem};

pub struct SummationOfPrimes {
    pub upper_bound: usize
}

impl Problem for SummationOfPrimes {
    fn solve(&self) -> String {
        let primes_below_bound = primes_below(self.upper_bound);
        let mut prime_sum = 0;

        let mut i = 0;
        while i < primes_below_bound.len() {
            if primes_below_bound[i] {
                prime_sum = prime_sum + i + 2;
            }

            i = i + 1;
        }

        return format!("{}", prime_sum);
    }
}