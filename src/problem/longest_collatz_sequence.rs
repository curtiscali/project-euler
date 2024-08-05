use std::collections::HashMap;

use super::Problem;

fn chain_length(n: u64, solutions: &mut HashMap<u64, u64>) -> u64 {
    let mut m = n;
    let mut length: u64 = 1;
    
    while m > 1 {
        if m % 2 == 0 {
            let half = m / 2;
            let existing_solution = solutions.get(&half);
            if existing_solution.is_some() {
                length += existing_solution.unwrap();
                break;
            } else {
                m /= 2;
            }
        } else {
            let mult = (3 * m) + 1;
            let existing_solution = solutions.get(&mult);
            if existing_solution.is_some() {
                length += existing_solution.unwrap();
                break;
            } else {
                m = mult;
            }
        }

        length += 1;
    }

    solutions.entry(n).and_modify(|x| *x = length);
    return length;
}

pub struct LongestCollatzSequenceProblem {
    pub limit: u32
}

impl Problem for LongestCollatzSequenceProblem {
    fn solve(&self) -> String {
        let mut solutions: HashMap<u64, u64> = HashMap::from([(1, 1)]);
        let mut longest_path = 1;
        let mut longest_path_num = 1;

        let mut i = 2;
        while i <= self.limit {
            let chain_length = chain_length(i as u64, &mut solutions);

            if chain_length > longest_path {
                longest_path = chain_length;
                longest_path_num = i;
            }

            i += 1;
        }

        return format!("{}", longest_path_num);
    }
}