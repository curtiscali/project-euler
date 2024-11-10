use core::f64;
use crate::{number_theory::to_digits, primes::primes_below};
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
        // sqrt(10^7) = 10^3.5 ~ 3200. using 3700 here accounts for a or b being slightly over that
        let primes = primes_below::<u64>(3700);
        let mut min_totient_n = u64::MAX;
        let mut min_totient_ratio = f64::INFINITY;

        for a in &primes {
            for b in &primes {
                // the best way to minimize n/phi(n) without using primes (phi(p) = p - 1, which
                // cannot be a permutation of p) is to use n = p1p2 where p1 & p2 are primes
                // a more specific version of the totient formula (https://mathworld.wolfram.com/TotientFunction.html)
                // for this case is n(p1 -1)(p2 - 1)
                // This speeds up the computations massively against brute force
                // when combined with sieving only to slightly above the sqrt(10^7)
                let n = a * b;
                let totient = (a - 1) * (b - 1);
                if n < 10_000_000 && is_permutation(n, totient) {
                    let totient_ratio = n as f64 / totient as f64;
                    if totient_ratio < min_totient_ratio {
                        min_totient_ratio = totient_ratio;
                        min_totient_n = n;
                    }
                }
            }
        }
        return format!("{}", min_totient_n);
    }
}
