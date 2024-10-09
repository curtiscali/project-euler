use crate::primes::sieve_of_atkin;
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

pub struct ReciprocalCyclesProblem {
    pub max_denominator: u32
}

impl Problem for ReciprocalCyclesProblem {
    fn solve(&self) -> String {
        let prime_denoms_lookup = sieve_of_atkin(self.max_denominator as usize);

        let mut max_repeat_length = 0;
        let mut max_repeat_denominator = 0;

        let mut d = self.max_denominator as usize - 1;
        while d > 2 {
            // Primes will have the longest repeating periods, so to optimize we can only check
            // primes
            if prime_denoms_lookup[d] {
                let repeat_length = multiplicative_order(d as u32);
                
                if repeat_length > max_repeat_length {
                    max_repeat_denominator = d;
                    max_repeat_length = repeat_length;
                }
            }
            
            d -= 1;
        }

        format!("{}", max_repeat_denominator)
    }
}
