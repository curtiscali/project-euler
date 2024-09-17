use super::Problem;
use crate::primes::sieve_of_atkin;

// F(limit) = sum(((n - 1) / 2) - (n / 3)) 
fn fraction_count(limit: usize) -> usize {
    let mut fraction_count = 0;
    let mut n = 1;
    while n <= limit {
        fraction_count += ((n - 1) / 2) - (n / 3);
        n += 1;
    }

    fraction_count
}

// Use the inclusion-exclusion principle to count fractions in a range
fn inclusion_exclusion(limit: usize, index: usize, primes: &Vec<usize>) -> usize {
    // count of all discrete fractions with d <= limit
    let mut count = fraction_count(limit);

    // For each computed prime:
    // find the new limit = 5 * primes[i]
    // subtract all the fractions that divide that prime from the total & do that in the next
    // index
    let mut i = index;
    while i < primes.len() && (5 * primes[i] <= limit) {
        let new_limit = limit / primes[i];
        count -= inclusion_exclusion(new_limit, i + 1, primes);
        i += 1;
    }

    return count;
}

pub struct CountingFractionsInARangeProblem {
    pub denom_limit: usize
}

impl Problem for CountingFractionsInARangeProblem {
    fn solve(&self) -> String {
        // The largest primes needed for inclusion/exclusion principle are the limit / 5
        // This reduces the needed computations
        // Fractions aren't reduced if a prime divides the numerator AND denominator]
        // That's what the primes are used for
        let primes_below_limit_lookup = sieve_of_atkin((self.denom_limit / 5) + 1);
        let mut primes_below_limit: Vec<usize> = vec![];
        let mut i = 0; 
        while i < primes_below_limit_lookup.len() {
            if primes_below_limit_lookup[i] {
                primes_below_limit.push(i);
            }

            i += 1;
        }

        return format!("{}", inclusion_exclusion(self.denom_limit, 0, &primes_below_limit));
    }
}
