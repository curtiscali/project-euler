use num::{BigInt, Integer};

use super::Problem;

fn factorial(n: u32) -> BigInt {
    let mut product = BigInt::from(1);

    let mut i = 2;
    while i <= n {
        product = product.checked_mul(&BigInt::from(i)).unwrap();
        i += 1;
    }

    return product;
}

pub struct FactorialDigitSum {
    pub n: u32
}

impl Problem for FactorialDigitSum {
    fn solve(&self) -> String {
        let ten = BigInt::from(10);
        let mut factorial = factorial(self.n);

        let mut digit_sum = BigInt::ZERO;
        while factorial.gt(&BigInt::ZERO) {
            digit_sum += factorial.mod_floor(&ten);
            factorial = factorial.div_floor(&ten);
        }

        return format!("{}", digit_sum);
    }
}