use super::Problem;
use crate::{number_theory::linear_sum, primes::prime_factors};

fn num_factors(n: u32) -> u32 {
    let prime_factors = prime_factors(n);
    prime_factors.into_iter().map(|(_, n)| n + 1).product()
}

pub struct HighlyDivisibleTriangleNumber {}

impl Problem for HighlyDivisibleTriangleNumber {
    fn name(&self) -> String {
        String::from("Highly Divisible Triangular Number")
    }

    fn number(&self) -> u16 {
        12
    }

    fn solve(&self) -> String {
        const TARGET_NUM_DIVISORS: u32 = 500;

        let mut n = 7;
        while num_factors(linear_sum(n)) <= TARGET_NUM_DIVISORS {
            n += 1;
        }

        return format!("{}", linear_sum(n));
    }
}
