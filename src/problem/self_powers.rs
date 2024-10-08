use num::BigInt;
use super::Problem;

pub struct SelfPowersProblem {
    pub upper_bound: u32
}

impl Problem for SelfPowersProblem {
    fn solve(&self) -> String {
        let modulus = BigInt::from(10_000_000_000u64);

        let mut self_power_sum = BigInt::ZERO;
        let mut i = 1;
        while i < self.upper_bound {
            let n = BigInt::from(i);
            self_power_sum += n.modpow(&n, &modulus);
            
            i += 1;
        }
        
        return format!("{}", self_power_sum % &modulus);
    }
}
