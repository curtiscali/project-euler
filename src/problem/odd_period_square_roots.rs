use num::integer::Roots;
use crate::number_theory::is_perfect_square;
use super::Problem;

// Algorithm based on https://en.wikipedia.org/wiki/Periodic_continued_fraction#Canonical_form_and_repetend
// Go until a = 2a0 to determine period
fn period_of_sqrt_fraction(n: u32) -> u32 {
    if is_perfect_square(n) {
        return 0;
    }

    let a0 = n.sqrt();
    let (mut m, mut d, mut a) = (0u32, 1u32, a0);
    let mut period = 0;

    while a != (2 * a0) {
        let next_m = (d * a) - m;
        let next_d = (n - (next_m * next_m)) / d;
        
        (a, m, d) = ((a0 + next_m) / next_d, next_m, next_d);

        period += 1;
    }

    period
}

pub struct OddPeriodSquareRootsProblem {}

impl Problem for OddPeriodSquareRootsProblem {
    fn solve(&self) -> String {
        let mut num_odd_period_roots = 0u32;

        for i in 2..=10000 {
            if period_of_sqrt_fraction(i) % 2 == 1 {
                num_odd_period_roots += 1;
            }
        }

        format!("{}", num_odd_period_roots)
    }
}
