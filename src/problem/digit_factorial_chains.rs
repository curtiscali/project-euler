use std::collections::{BTreeSet, HashMap};

use super::Problem;

fn digit_factorial_sum(n: u64) -> u64 {
    let factorial_values = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    let mut digit_factorial_sum = 0;
    let mut num = n;
    while num > 0 {
        let digit = (num % 10) as usize;
        digit_factorial_sum += factorial_values[digit];
        num /= 10;
    }

    digit_factorial_sum
}

fn chain_length(n: u64) -> u64 {
    let known_loops = HashMap::from([
        (1u64, 1u64),
        (2, 1),
        (145, 1),
        (169, 3),
        (871, 2),
        (872, 2),
        (1454, 3),
        (40585, 1),
        (45361, 2),
        (45362, 2),
        (363601, 3)
    ]);

    if known_loops.contains_key(&n) {
        return known_loops[&n];
    }

    let mut chain_length = 0;
    let mut x = n;
    while !known_loops.contains_key(&x) {
        chain_length += 1;
        x = digit_factorial_sum(x);
    }

    chain_length + known_loops[&x]
}

pub struct DigitFactorialChainsProblem {}

impl Problem for DigitFactorialChainsProblem {
    fn solve(&self) -> String {
        let mut num_chains_with_len_sixty = 0;

        for i in 2..1_000_000 {
            if chain_length(i) == 60 {
                num_chains_with_len_sixty += 1;
            }
        }

        format!("{}", num_chains_with_len_sixty)
    }
}
