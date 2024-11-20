use num::integer::Roots;
use crate::primes::primes_below;
use super::Problem;

fn phi(x: u64, a: u64, primes: &Vec<u64>) -> u64 {
    if a < 1 {
        return x;
    }

    if a == 1 {
        return x - (x >> 1);
    }

    let pa = primes[(a - 1) as usize];
    if x <= pa {
        return 1;
    }

    phi(x, a - 1, primes) - phi(x / pa, a - 1, primes)
}

fn pi(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    let primes = primes_below(n.sqrt() as usize);
    let a = primes.len() as u64;

    phi(n, a, &primes) + a - 1
}

pub struct SemiprimesProblem {}

impl Problem for SemiprimesProblem {
    fn name(&self) -> String {
        String::from("Semiprimes")
    }

    fn number(&self) -> u16 {
        187
    }

    fn solve(&self) -> String {
        const N: u64 = 100_000_000;

        // This function implements the Semiprime counting function https://en.wikipedia.org/wiki/Semiprime#Formula_for_number_of_semiprimes
        let root = N.sqrt();
        let primes = primes_below(root as usize);

        let mut num_semiprimes = 0u64;
        for k in 1..=primes.len() {
            let p = primes[k - 1];
            num_semiprimes += pi(N / p) - (k as u64) + 1;
        }

        format!("{}", num_semiprimes)
    }
}
