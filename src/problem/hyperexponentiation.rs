use super::Problem;
use crate::hyperops::mod_tetr;

pub struct HyperexponentiationProblem { 
    pub a: u64,
    pub b: u64
}

impl Problem for HyperexponentiationProblem {
    fn solve(&self) -> String {
        let modulus = 100_000_000;
        format!("{}", mod_tetr(self.a, self.b, modulus))
    }
}
