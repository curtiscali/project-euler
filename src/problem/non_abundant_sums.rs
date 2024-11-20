use std::collections::BTreeSet;
use crate::{number_theory::fast_pow, primes::prime_factors};
use super::Problem;

const MAX_NON_ABUNDANT: u64 = 28123;

// This constant is based on the wikipedia article: https://en.wikipedia.org/wiki/Abundant_number
// all numbers greater than this CAN be written as the sum of two abundant numbers
const MAX_NON_ABUNDANT_SUM: u64 = 20161;

fn sigma(n: u64) -> u64 {
    let prime_factors = prime_factors(n);
    let mut divisors_sum = 1;

    for (factor, power) in prime_factors {
        divisors_sum *= (fast_pow(factor, power + 1) - 1) / (factor - 1);
    }

    divisors_sum
}

fn is_abundant_sum(n: u64, lookup: &BTreeSet<u64>) -> bool {
    if n > MAX_NON_ABUNDANT {
        return true;
    }

    for i in lookup {
        if *i >= n {
            break;
        }

        let j = n - i;
        if !lookup.contains(&j) {
            continue;
        }

        return true;
    }

    false
}

pub struct NonAbundantSumsProblem {}

impl Problem for NonAbundantSumsProblem {
    fn name(&self) -> String {
        String::from("Non-Abundant Sums")
    }

    fn number(&self) -> u16 {
        23
    }

    fn solve(&self) -> String {
        let mut abundant_numbers: BTreeSet<u64> = BTreeSet::new();

        let mut n = 1;
        while n <= MAX_NON_ABUNDANT {
            if !abundant_numbers.contains(&n) {
                let is_abundant = sigma(n) > 2 * n;

                if is_abundant {
                    abundant_numbers.insert(n);
                    let mut m = n * 2;
                    while m <= MAX_NON_ABUNDANT {
                        abundant_numbers.insert(m);
                        m += n;
                    }
                }
            }

            n += 1;
        }

        let mut sum = 0;
        let mut n = 1;
        while n <= MAX_NON_ABUNDANT_SUM {
            if !is_abundant_sum(n, &abundant_numbers) {
                sum += n;
            }

            n += 1;
        }

        format!("{}", sum)
    }
}
