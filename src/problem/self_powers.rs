use num::BigInt;
use super::Problem;

pub struct SelfPowersProblem {
    pub upper_bound: u32
}

impl Problem for SelfPowersProblem {
    fn solve(&self) -> String {
        let mut sum = BigInt::from(0);
        let mut i = 1;
        while i < self.upper_bound {
            let self_power = BigInt::from(i).pow(i);
            sum += self_power;
            
            i += 1;
        }
        
        return format!("{}", sum % BigInt::from(10_000_000_000usize));
    }
}