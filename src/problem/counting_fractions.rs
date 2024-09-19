use super::Problem;
use crate::primes::sieve_of_atkin;

fn fraction_count(limit: u32) -> u128 {
    let n = limit as u128;
    ((n * (n + 1)) / 2) - n
}

// Use the inclusion-exclusion principle to count fractions in a range
fn inclusion_exclusion(limit: u32, index: usize, primes: &Vec<u32>) -> u128 {
    // count of all discrete fractions with d <= limit
    let mut count = fraction_count(limit);

    // For each computed prime:
    // subtract all the fractions that divide that prime from the total & do that in the next
    let mut i = index;
    while i < primes.len() && (2 * primes[i]) <= limit {
        let new_limit = limit / primes[i];
        count -= inclusion_exclusion(new_limit, i + 1, primes);
        i += 1;
    }

    return count;
}

pub struct CountingFractionsProblem {
    pub denom_limit: u32
}

impl Problem for CountingFractionsProblem {
    fn solve(&self) -> String {
        // Fractions aren't reduced if a prime divides the numerator AND denominator]
        // That's what the primes are used for
        let primes_below_limit_lookup = sieve_of_atkin((self.denom_limit as usize) / 2);
        let mut primes_below_limit: Vec<u32> = vec![];
        let mut i = 0; 
        while i < primes_below_limit_lookup.len() {
            if primes_below_limit_lookup[i] {
                primes_below_limit.push(i as u32);
            }

            i += 1;
        }

        return format!("{}", inclusion_exclusion(self.denom_limit, 0, &primes_below_limit));
    }
}
