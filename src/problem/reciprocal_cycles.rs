use crate::{arithmetic::{fast_pow, gcd, lcm}, primes::{prime_factors, sieve_of_atkin}};
use super::Problem;

fn multiplicative_order(n: u32) -> u32 {
    let mut result = 1u32;

    let mut k = 1;
    while k < 10 {
        result = (result * n) % 10;

        if result == 1 {
            return k;
        }

        k += 1;
    }

    0
}

fn repeat_length(n: u32, primes_lookup: &Vec<bool>) -> u32 {
    if primes_lookup[n as usize] {
        return n - 1;
    }

    // if n is coprime to 10
    if gcd(n, 10) == 1 {
       let prime_factors = prime_factors(n);
       let periods = prime_factors.into_iter()
           .map(|(p, k)| fast_pow(p, k - 1) * (p - 1))
           .collect::<Vec<u32>>();

        return lcm(&periods);
    }

    let prime_factors = prime_factors(n);
    let periods = prime_factors.into_iter()
        .filter(|(p, _)| *p != 2 && *p != 5)
        .map(|(p, k)| fast_pow(p, k - 1) * (p - 1))
        .collect::<Vec<u32>>();
    
    if periods.len() == 0 {
        return 0;
    }

    lcm(&periods)
}

pub struct ReciprocalCyclesProblem {
    pub max_denominator: u32
}

impl Problem for ReciprocalCyclesProblem {
    fn solve(&self) -> String {
        let prime_denoms_lookup = sieve_of_atkin(self.max_denominator as usize);

        let mut max_repeat_length = 0;

        let mut d = self.max_denominator - 1;
        while d > 2 {
            let repeat_length = repeat_length(d, &prime_denoms_lookup);
            
            if repeat_length > max_repeat_length {
                max_repeat_length = repeat_length;
            }

            d -= 1;
        }

        format!("{}", max_repeat_length)
    }
}
