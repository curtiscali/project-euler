use super::Problem;
use num::BigInt;

pub struct LargeNonMersennePrimeProblem {}

impl Problem for LargeNonMersennePrimeProblem {
    fn name(&self) -> String {
        String::from("Large Non-Mersenne Prime")
    }

    fn number(&self) -> u16 {
        97
    }

    fn solve(&self) -> String {
        let prime = BigInt::from(28433).checked_mul(
            &BigInt::from(2).pow(7830457)
        ).unwrap();

        return format!("{}", (prime + 1) % 10000000000usize);
    }
}
