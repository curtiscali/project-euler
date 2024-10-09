use std::collections::HashMap;
use crate::{arithmetic::{fast_pow, gcd, lcm}, primes::{prime_factors, sieve_of_atkin}};
use super::Problem;

fn multiplicative_order(n: u32) -> u32 {
    let mut result = 1u32;

    let mut k = 1;
    while k < n {
        result = (result * 10) % n;

        if result == 1 {
            return k;
        }

        k += 1;
    }

    0
}

fn repeat_length(n: u32, primes_periods_lookup: &HashMap<u32, u32>) -> u32 {
    match primes_periods_lookup.get(&n) {
        Some(period) => return *period,
        None => {
            let prime_factors = prime_factors(n);
            // if n is coprime to 10
            if gcd(n, 10) == 1 {
               let periods = prime_factors.into_iter()
                   .map(|(p, k)| fast_pow(p, k - 1) * primes_periods_lookup[&p])
                   .collect::<Vec<u32>>();

                return lcm(&periods);
            }

            let non_two_or_five_factors_product = prime_factors.into_iter()
                .filter(|(p, _)| *p != 2 && *p != 5)
                .map(|(p, k)| fast_pow(p, k))
                .product::<u32>();
            
            if non_two_or_five_factors_product == 1 {
                return 0;
            }

            return multiplicative_order(non_two_or_five_factors_product);
        }
    }
}

pub struct ReciprocalCyclesProblem {
    pub max_denominator: u32
}

impl Problem for ReciprocalCyclesProblem {
    fn solve(&self) -> String {
        let prime_denoms_lookup = sieve_of_atkin(self.max_denominator as usize);
        let mut prime_periods: HashMap<u32, u32> = HashMap::new();

        let mut n = 2;
        while n < prime_denoms_lookup.len() {
            if prime_denoms_lookup[n] {
                prime_periods.insert(n as u32, multiplicative_order(n as u32));
            }

            n += 1;
        }

        let mut max_repeat_length = 0;
        let mut max_repeat_denominator = 0;

        let mut d = self.max_denominator - 1;
        while d > 2 {
            let repeat_length = repeat_length(d, &prime_periods);
            
            if repeat_length > max_repeat_length {
                max_repeat_denominator = d;
                max_repeat_length = repeat_length;
            }

            d -= 1;
        }

        format!("{}", max_repeat_denominator)
    }
}
