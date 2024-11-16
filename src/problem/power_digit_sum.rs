use num::BigInt;
use crate::number_theory::bigint_digit_sum;

use super::Problem;

pub struct PowerDigitSum {}

impl Problem for PowerDigitSum {
    fn name(&self) -> String {
        String::from("Power Digit Sum")
    }

    fn number(&self) -> u16 {
        16
    }

    fn solve(&self) -> String {
        let value = BigInt::from(2).pow(1000);
        return format!("{}", bigint_digit_sum(&value));
    }
}
