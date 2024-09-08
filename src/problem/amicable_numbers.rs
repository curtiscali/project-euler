use std::collections::HashMap;

use crate::primes::sieve_of_atkin;

use super::Problem;

fn proper_divisors_sum(n: usize) -> usize {
    let mut factor_sum = 1;

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            factor_sum += i + (n / i);
        }

        i += 1;
    }

    return factor_sum;
}

pub struct AmicableNumbersProblem {
    pub limit: usize
}

impl Problem for AmicableNumbersProblem {
    fn solve(&self) -> String {
        let mut amicable_numbers_sum = 0;

        let mut i = 2;
        while i < self.limit {
            let j = proper_divisors_sum(i);

            if j > i && proper_divisors_sum(j) == i {
                amicable_numbers_sum += i + j;
            }

            i += 1;
        }

        return format!("{}", amicable_numbers_sum);
    }
}
