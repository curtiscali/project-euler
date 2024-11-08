use core::f64;
use crate::arithmetic::{totient, to_digits};
use super::Problem;

fn is_permutation(a: u64, b: u64) -> bool {
    let mut a_digits = to_digits(a);
    let mut b_digits = to_digits(b);

    a_digits.sort();
    b_digits.sort();

    a_digits == b_digits
}

pub struct TotientPermutationProblem {}

impl Problem for TotientPermutationProblem {
    fn solve(&self) -> String {
        let mut min_totient_n = u64::MAX;
        let mut min_totient_ratio = f64::INFINITY;

        for n in 2..10_000_000 {
            let totient = totient(n);

            if is_permutation(n, totient) {
                let totient_ratio = n as f64 / totient as f64;
                if totient_ratio < min_totient_ratio {
                    min_totient_ratio = totient_ratio;
                    min_totient_n = n;
                }
            }
        }

        return format!("{}", min_totient_n);
    }
}
