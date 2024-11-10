use crate::number_theory::{reverse, is_palindrome};
use super::Problem;

fn is_lychrel_number(n: u128) -> bool {
    const MAX_LYCHREL_ITERATIONS: u32 = 50;

    let mut x = n;
    for _ in 0..MAX_LYCHREL_ITERATIONS {
        let number = x + reverse(x);
        if is_palindrome(number) {
            return false;
        }

        x = number;
    }

    return true;
}

pub struct LychrelNumbersProblem {}

impl Problem for LychrelNumbersProblem {
    fn solve(&self) -> String {
        let mut lychrel_numbers_count = 0;

        let mut i = 1;
        while i <= 10_000 {
            if is_lychrel_number(i as u128) {
                lychrel_numbers_count += 1;
            }

            i += 1;
        }

        return format!("{}", lychrel_numbers_count);
    }
}
