use crate::combinatorics::partitions_count;

use super::Problem;

pub struct CoinSumProblem {}

impl Problem for CoinSumProblem {
    fn name(&self) -> String {
        String::from("Coin Sum")
    }

    fn number(&self) -> u16 {
        31
    }

    fn solve(&self) -> String {
        const TOTAL_PENCE: u64 = 200;
        let denominations = vec![1u64, 2, 5, 10, 20, 50, 100, 200];

        return format!("{}", partitions_count(TOTAL_PENCE, &denominations));
    }
}
