use std::collections::BTreeSet;
use num::BigInt;
use super::Problem;

pub struct DistinctPowersProblem {}

impl Problem for DistinctPowersProblem {
    fn solve(&self) -> String {
        const MAX_POWER_AND_BASE: u32 = 100;

        let mut existing_exponents: BTreeSet<BigInt> = BTreeSet::new();
        for a in 2..=MAX_POWER_AND_BASE {
            for b in 2..=MAX_POWER_AND_BASE {
                let evaluated = BigInt::from(a).pow(b);
                if !existing_exponents.contains(&evaluated) {
                    existing_exponents.insert(evaluated);
                }
            }
        }

        return format!("{}", existing_exponents.len());
    }
}
