use crate::arithmetic::bigint_num_digits;

use super::Problem;
use num::{BigInt, FromPrimitive};

pub struct ThousandDigitFibonacciNumberProblem {}

impl Problem for ThousandDigitFibonacciNumberProblem {
    fn solve(&self) -> String {
        let one_thousand = BigInt::from_u16(1000).unwrap();
        let mut fib1 = BigInt::from(1);
        let mut fib2 = BigInt::from(1);
        let mut fib_index = 2;

        while bigint_num_digits(&fib2).lt(&one_thousand) {
            let new_fib = &fib1 + &fib2;
            fib1 = fib2.clone();
            fib2 = new_fib;

            fib_index += 1;
        }

        return format!("{}", fib_index);
    }
}
