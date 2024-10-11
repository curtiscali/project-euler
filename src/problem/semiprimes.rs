use crate::primes::sieve_of_atkin;
use super::Problem;

fn pi(n: u32, primes: &Vec<u32>) -> u32 {
    let mut primes_count = 0;
    let mut i = 0;
    while i < primes.len() {
        if primes[i] > n {
            break;
        }

        primes_count += 1;

        i += 1;
    }

    primes_count
}

pub struct SemiprimesProblem {
    pub n: u32
}

impl Problem for SemiprimesProblem {
    fn solve(&self) -> String {
        // This function implements teh Semiprime counting function https://en.wikipedia.org/wiki/Semiprime#Formula_for_number_of_semiprimes
        let primes_below_half_lookup = sieve_of_atkin((self.n / 2) as usize);
        let mut primes_below_half: Vec<u32> = vec![];

        let mut i = 2;
        while i < primes_below_half_lookup.len() {
            if primes_below_half_lookup[i] {
                primes_below_half.push(i as u32);
            }

            i += 1;
        }

        let pi_sqrt = pi((self.n as f64).sqrt() as u32, &primes_below_half);

        let mut num_semiprimes = 0u32;
        let mut k = 1;
        while k <= pi_sqrt {
            let p = primes_below_half[(k as usize) - 1];

            num_semiprimes += pi(self.n / p, &primes_below_half) - k + 1;

            k += 1;
        }

        format!("{}", num_semiprimes)
    }
}
