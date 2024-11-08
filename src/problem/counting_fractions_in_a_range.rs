use super::Problem;
use crate::primes::primes_below;

// F(limit) = sum(((n - 1) / 2) - (n / 3)) 
fn fraction_count(limit: u32) -> u32 {
    let mut fraction_count = 0;
    let mut n = 1;
    while n <= limit {
        fraction_count += ((n - 1) / 2) - (n / 3);
        n += 1;
    }

    fraction_count
}

// Use the inclusion-exclusion principle to count fractions in a range
fn inclusion_exclusion(limit: u32, index: usize, primes: &Vec<u32>) -> u32 {
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

pub struct CountingFractionsInARangeProblem {}

impl Problem for CountingFractionsInARangeProblem {
    fn solve(&self) -> String {
        const DENOM_LIMIT: u32 = 12_000;

        // The largest primes needed for inclusion/exclusion principle are the limit / 5
        // This reduces the needed computations
        // Fractions aren't reduced if a prime divides the numerator AND denominator]
        // That's what the primes are used for
        let primes_below_limit = primes_below::<u32>(((DENOM_LIMIT / 5) + 1) as usize);

        format!("{}", inclusion_exclusion(DENOM_LIMIT, 0, &primes_below_limit))
    }
}
