use num::BigInt;

use crate::combinatorics::bigint_combinations;

use super::Problem;

pub struct NonBouncyNumbersProblem {}

impl Problem for NonBouncyNumbersProblem {
    fn solve(&self) -> String {
        let nine = BigInt::from(9);
        let ten = BigInt::from(10);

        let k = BigInt::from(100);
        let num_increasing = bigint_combinations(&(&k + &nine), &nine) - 1;
        let num_decreasing = bigint_combinations(&(&k + &ten), &ten) - &(&k + 1);
        let num_both = &nine * &k;

        format!("{}", &num_increasing + &num_decreasing - &num_both)
    }
}