use std::collections::HashMap;
use crate::primes::prime_factors;
use super::Problem;

fn distinct_prime_factors(n: u32) -> Vec<u32> {
    let all_prime_factors = prime_factors(n);
    let mut factors_hash: HashMap<u32, bool> = HashMap::new();

    for factor in all_prime_factors {
        factors_hash.entry(factor)
            .and_modify(|x| *x = true)
            .or_insert(true);
    }

    return factors_hash.into_keys().into_iter().collect::<Vec<u32>>();
}

pub struct DistinctPrimeFactorsProblem {}

impl Problem for DistinctPrimeFactorsProblem {
    fn solve(&self) -> String {
        const TARGET_FACTORS_COUNT: usize = 4;

        let mut found_numbers: Vec<u32> = vec![];
        let mut n = 647;

        while found_numbers.len() < TARGET_FACTORS_COUNT {
            let distinct_prime_factors = distinct_prime_factors(n);

            if distinct_prime_factors.len() == TARGET_FACTORS_COUNT {
                if found_numbers.len() == 0 {
                    found_numbers.push(n);
                } else {
                    if n - found_numbers[found_numbers.len() - 1] != 1 {
                        found_numbers.clear();
                    }

                    found_numbers.push(n);
                }
            }

            n += 1;
        }


        format!("{:?}", found_numbers)
    }
}
