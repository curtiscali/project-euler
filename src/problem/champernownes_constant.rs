use num::pow;

use crate::arithmetic::num_digits;

use super::Problem;

fn nth_digit(num: usize, digit_num: usize) -> usize {
    let mut n = num;
    let mut curr_digit = num_digits(n);
    
    let mut digit = n % 10;
    n /= 10;
    curr_digit -= 1;

    while n > 0 && curr_digit > digit_num {
        digit = n % 10;
        curr_digit -= 1;
        n /= 10;
    }

    return digit;
}

fn champernowne_digit(n: usize) -> usize {
    let mut total_digits = 1;
    let mut current = 1;
    while total_digits < n {
        total_digits += num_digits(current);
        current += 1;
    }

    let offset = total_digits - n;

    return nth_digit(current, offset)
}

pub struct ChampernownesConstantProblem {}

impl Problem for ChampernownesConstantProblem {
    fn solve(&self) -> String {
        let mut product: usize = 1;
        let mut i: usize = 0;
        while i < 7 {
            product *= champernowne_digit(pow(10, i));
            i += 1;
        }

        return format!("{}", product);
    }
}