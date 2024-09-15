use crate::arithmetic::{reverse, is_palindrome};
use super::Problem;

fn is_lychrel_number(n: u128) -> bool {
    const MAX_LYCHREL_ITERATIONS: u32 = 50;

    let mut i = 0;
    let mut x = n;
    while i < MAX_LYCHREL_ITERATIONS {
        let number = x + reverse(x);
        if is_palindrome(number) {
            return false;
        }

        x = number;
        i += 1;
    }

    return true;
}

pub struct LychrelNumbersProblem {
    pub upper_bound: usize
}

impl Problem for LychrelNumbersProblem {
    fn solve(&self) -> String {
        let mut lychrel_numbers_count = 0;

        let mut i = 1;
        while i <= self.upper_bound {
            if is_lychrel_number(i as u128) {
                lychrel_numbers_count += 1;
            }

            i += 1;
        }

        return format!("{}", lychrel_numbers_count);
    }
}
