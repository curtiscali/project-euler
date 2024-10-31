use crate::combinatorics::partitions_count;
use super::Problem;

pub struct CountingSummationsProblem {}

impl Problem for CountingSummationsProblem {
    fn solve(&self) -> String {
        const N: u64 = 100;
        let partitions = (1..N).collect::<Vec<u64>>();

        format!("{}", partitions_count(N, &partitions))
    }
}