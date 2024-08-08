use std::collections::BTreeSet;
use num::BigInt;

use super::Problem;

pub struct DistinctPowersProblem {
    pub upper_bound: u32
}

impl Problem for DistinctPowersProblem {
    fn solve(&self) -> String {
        let mut existing_exponents: BTreeSet<BigInt> = BTreeSet::new();
        let mut a = 2;
        while a <= self.upper_bound {
            let mut b = 2;
            while b <= self.upper_bound {
                let evaluated = BigInt::from(a).pow(b);
                if !existing_exponents.contains(&evaluated) {
                    existing_exponents.insert(evaluated);
                }

                b += 1;
            }

            a += 1;
        }

        return format!("{}", existing_exponents.len());
    }
}