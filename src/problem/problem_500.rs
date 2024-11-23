use std::{cmp::Reverse, collections::BinaryHeap};
use num::{BigInt, One};
use super::Problem;
use crate::primes::primes_below;

pub struct Problem500 {}

impl Problem for Problem500 {
    fn name(&self) -> String {
        String::from("Problem 500!!!")
    }

    fn number(&self) -> u16 {
        500
    }

    // Thank you StackOverflow: code based on this algo:
    // https://stackoverflow.com/a/31271590
    fn solve(&self) -> String {
        const N: usize = 500500;

        // Using the asymptotic bounds nln(n)+n(ln(ln(n))−1) < pn < nln(n)+nln(ln(n)) for n >= 6
        // We can generate a list of primes below that upper bound for the nth primes
        // then look at index n - 1 to find our target prime
        let float_n = N as f64;
        let ln = float_n.ln();
        let float_upper_bound = (float_n * ln) + (float_n * ln.ln());
        let upper_bound = float_upper_bound as usize;

        let mut primes_heap = primes_below(upper_bound).into_iter()
            .map(|p| Reverse(p))
            .collect::<BinaryHeap<Reverse<u64>>>();

        let modulus = 500500507u64;
        let mut x = BigInt::one();

        let mut current_power_of_two = 0u32;
        let target_power_of_two = N as u32;
        while current_power_of_two < target_power_of_two {
            let p = match primes_heap.pop() {
                Some(Reverse(pi)) => pi,
                None => 0
            };

            x = (x % modulus) * (p % modulus);

            primes_heap.push(Reverse(p * p));

            current_power_of_two += 1;
        }

        format!("{}", x % modulus)
    }
}
