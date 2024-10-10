use crate::primes::sieve_of_atkin;

use super::Problem;

fn spf_sieve(n: usize) -> Vec<usize> {
    let mut smallest_prime_factors = vec![1; n + 1];

    let mut i = 2;
    while i <= n {
        if smallest_prime_factors[i] == 1 {
            let mut j = i;
            while j <= n {
                if smallest_prime_factors[j] == 1 {
                    smallest_prime_factors[j] = i;
                }

                j += i;
            }
        }

        i += 1;
    }

    return smallest_prime_factors;
}

fn is_semiprime(n: usize, smallest_prime_factors: &Vec<usize>) -> bool {
    let mut factors_count = 0;

    let mut x = n;
    while x != 1 {
        factors_count += 1;
        x /= smallest_prime_factors[x];
    }

    factors_count == 2
}

pub struct SemiprimesProblem {
    pub upper_bound: u32
}

impl Problem for SemiprimesProblem {
    fn solve(&self) -> String {
        let primes_below_bound = sieve_of_atkin(self.upper_bound as usize);
        let smallest_prime_factors = spf_sieve(self.upper_bound as usize);

        let mut num_semiprime = 0;
        let mut i = 2;
        while i < primes_below_bound.len() {
            if !primes_below_bound[i] && is_semiprime(i, &smallest_prime_factors) {
                num_semiprime += 1;
            }

            i += 1;
        }

        format!("{}", num_semiprime)
    }
}
