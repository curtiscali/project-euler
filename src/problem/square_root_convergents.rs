use num::BigInt;
use crate::number_theory::bigint_num_digits;
use super::Problem;

pub struct SquareRootConvergentsProblem {}

impl Problem for SquareRootConvergentsProblem {
    fn name(&self) -> String {
        String::from("Square Root Convergents")
    }

    fn number(&self) -> u16 {
        57
    }

    fn solve(&self) -> String {
        let (mut a, mut b, mut num_qualified) = (BigInt::from(1), BigInt::from(1), 0u32);

        for _ in 0..1000 {
            let (next_a, next_b) = ((2 * &b) + &a, &b + &a);

            a = next_a;
            b = next_b;

            if bigint_num_digits(&a) > bigint_num_digits(&b) {
                num_qualified += 1;
            }
        }

        format!("{}", num_qualified)
    }
}
