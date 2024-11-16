use num::integer::Roots;
use super::Problem;

fn partitions(n: u64) -> Vec<Vec<u64>> {
    let mut partitions = vec![];

    partitions
}

fn is_s_number(n: u64) -> bool {
    true
}

pub struct NumberSplittingProblem {}

impl Problem for NumberSplittingProblem {
    fn name(&self) -> String {
        String::from("Number Splitting")
    }

    fn number(&self) -> u16 {
        719
    }

    fn solve(&self) -> String {
        const N: u64 = 1_000_000_000_000;
        let root = N.sqrt();
        let mut s_nums_sum = 0u64;

        for i in 4..=root {
            let square = i * i;
        }

        format!("{}", s_nums_sum)
    }
}
