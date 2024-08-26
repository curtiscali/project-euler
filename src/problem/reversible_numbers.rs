use crate::arithmetic::reverse_usize;

use super::Problem;

fn are_all_digits_odd(n: usize) -> bool {
    let mut x = n;

    while x > 0 {
        let digit = x % 10;

        if digit % 2 == 0 {
            return false;
        }

        x /= 10;
    }
    
    return true;
}

pub struct ReversibleNumbersProblem {
    pub upper_bound: usize
}

impl Problem for ReversibleNumbersProblem {
    fn solve(&self) -> String {
        let mut num_reversibles = 0;
        let mut i = 1;
        while i < self.upper_bound {
            if i % 10 != 0 {
                let rev = reverse_usize(i);
                let rev_sum = i + rev;

                if are_all_digits_odd(rev_sum) {
                    num_reversibles += 1;
                }
            }

            i += 1;
        }

        return format!("{}", num_reversibles);
    }
}