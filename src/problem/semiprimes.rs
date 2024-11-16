use num::integer::Roots;
use crate::primes::primes_below;
use super::Problem;

fn phi(x: u32, a: u32, primes: &Vec<u32>) -> u32 {
    if a < 1 {
        return x;
    }

    if a == 1 {
        return x - (x >> 1);
    }

    let pa = primes[(a as usize) - 1];
    if x <= pa {
        return 1;
    }

    phi(x, a - 1, primes) - phi(x / pa, a - 1, primes)
}

fn pi(n: u32) -> u32 {
    if n < 2 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    let primes = primes_below::<u32>(n.sqrt() as usize);
    let a = primes.len() as u32;

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
        const N: u32 = 100_000_000;

        // This function implements the Semiprime counting function https://en.wikipedia.org/wiki/Semiprime#Formula_for_number_of_semiprimes
        let root = N.sqrt();
        let primes = primes_below::<u32>(root as usize);

        let mut num_semiprimes = 0u32;
        for k in 1..=primes.len() {
            let p = primes[k - 1];
            num_semiprimes += pi(N / p) - (k as u32) + 1;
        }

        format!("{}", num_semiprimes)
    }
}
