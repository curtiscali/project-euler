use num::BigInt;
use super::Problem;
use crate::hyperops::mod_tetr;

pub struct HyperexponentiationProblem {}

impl Problem for HyperexponentiationProblem {
    fn solve(&self) -> String {
        let modulus = BigInt::from(100_000_000);
        format!("{}", mod_tetr(&BigInt::from(1777), &BigInt::from(1855), &modulus))
    }
}
