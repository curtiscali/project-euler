use num::{BigInt, Integer};

use crate::Problem;

pub struct PowerDigitSum {}

impl Problem for PowerDigitSum {
    fn solve(&self) -> String {
        let ten = BigInt::from(10);

        let mut value = BigInt::from(2).pow(1000);

        let mut digit_sum = BigInt::ZERO;
        while value.cmp(&BigInt::ZERO).is_gt() {
            digit_sum = digit_sum.checked_add(&value.mod_floor(&ten)).unwrap();
            value = value.checked_div(&ten).unwrap();
        }

        return format!("{}", digit_sum);
    }
}