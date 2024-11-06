use num::BigInt;
use crate::combinatorics::bigint_combinations;
use super::Problem;

pub struct NonBouncyNumbersProblem {}

impl Problem for NonBouncyNumbersProblem {
    fn solve(&self) -> String {
        let nine = BigInt::from(9);
        let ten = BigInt::from(10);

        // Based on combinatoric magic that allows us to easily compute the number of increasing/decreasing numbers below a given 10^k
        // https://www.ttested.com/bouncy-numbers/
        // TL;DR - we can look at numbers as a set of digits, and therefore we can select combinations that are increasing
        // or decreasing

        // For an even better explanation: https://math.stackexchange.com/a/10849

        // exponent of 10 for which we want to find the number of non-bouncy numbers
        // In this case, 10^100, so k = 100
        let k = BigInt::from(100);

        // Bouncy numbers are numbers that are NEITHER increasing NOR decreasing
        // Therefore if we add the number of inc & decr numbers and avoid counting both
        // We can calculate the number of bouncy numbers below 10^k
        let num_increasing = bigint_combinations(&(&k + &nine), &nine) - 1;
        let num_decreasing = bigint_combinations(&(&k + &ten), &ten) - &(&k + 1);
        let num_both = &nine * &k;

        format!("{}", &num_increasing + &num_decreasing - &num_both)
    }
}