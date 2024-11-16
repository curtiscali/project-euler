use num::BigInt;
use super::Problem;

pub struct SelfPowersProblem {}

impl Problem for SelfPowersProblem {
    fn name(&self) -> String {
        String::from("Self Powers")
    }

    fn number(&self) -> u16 {
        48
    }

    fn solve(&self) -> String {
        let modulus = BigInt::from(10_000_000_000u64);

        let mut self_power_sum = BigInt::ZERO;
        for i in 1..1000 {
            let n = BigInt::from(i);
            self_power_sum += n.modpow(&n, &modulus);
        }
        
        return format!("{}", self_power_sum % &modulus);
    }
}
