use core::f64;
use std::f64::consts::PI;
use crate::primes::prime_factors;

use super::BonusProblem;

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

fn is_squarefree(n: u64) -> bool {
    let prime_factors = prime_factors(n);
    prime_factors.into_iter().all(|(_, n)| n == 1)
}

pub struct HeegnerProblem {}

impl BonusProblem for HeegnerProblem {
    fn name(&self) -> String {
        String::from("Heegner")
    }

    fn solve(&self) -> String {
        let non_square_numbers = (2..=1000).filter(|n| is_squarefree(*n)).collect::<Vec<u64>>();

        let mut min_int_diff = f64::MAX;
        let mut min_n = 0;

        for n in non_square_numbers {
            let x = (n as f64).sqrt() * PI;
            let cos = x.cos();

            let min_diff = min(cos - cos.floor(), cos.ceil() - cos);
            if min_diff < min_int_diff {
                min_int_diff = min_diff;
                min_n = n;
            }
        }

        format!("{}", min_n)
    }
}
