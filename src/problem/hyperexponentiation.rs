use num::BigInt;

use super::Problem;
use crate::hyperops::mod_tetr;

pub struct HyperexponentiationProblem { 
    pub a: u64,
    pub b: u64
}

impl Problem for HyperexponentiationProblem {
    fn solve(&self) -> String {
        let modulus = BigInt::from(100_000_000);
        format!("{}", mod_tetr(&BigInt::from(self.a), &BigInt::from(self.b), &modulus))
    }
}
