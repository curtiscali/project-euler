use crate::number_theory::to_digits;
use super::Problem;

pub struct PermutedMultiplesProblem {}

impl Problem for PermutedMultiplesProblem {
    fn solve(&self) -> String {
        let mut i: u64 = 125_875;
        let mut found_number = false;
        let mut matching_number = 125_874;

        while !found_number {
            
            let mut digits = to_digits(i);
            digits.sort();

            let mut digits_2n = to_digits(2 * i);
            digits_2n.sort();

            let mut digits_3n = to_digits(3 * i);
            digits_3n.sort();

            let mut digits_4n = to_digits(4 * i);
            digits_4n.sort();

            let mut digits_5n = to_digits(5 * i);
            digits_5n.sort();

            let mut digits_6n = to_digits(6 * i);
            digits_6n.sort();

            if digits == digits_2n && digits == digits_3n && digits == digits_4n && digits == digits_5n && digits == digits_6n {
                found_number = true;
                matching_number = i;
            }

            i += 1;
        }

        return format!("{}", matching_number);
    }
}
