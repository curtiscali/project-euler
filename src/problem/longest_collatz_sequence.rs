use std::collections::HashMap;

use super::Problem;

fn chain_length(n: u64, solutions: &mut HashMap<u64, u64>) -> u64 {
    let mut m = n;
    let mut length: u64 = 1;
    
    while m > 1 {
        if m % 2 == 0 {
            let half = m / 2;

            match solutions.get(&half) {
                Some(existing_solution) => {
                    length += *existing_solution;
                    break;
                },
                _ => m /= 2
            }
        } else {
            let mult = (3 * m) + 1;

            match solutions.get(&mult) {
                Some(existing_solution) => {
                    length += *existing_solution;
                    break;
                },
                _ => m = mult
            }
        }

        length += 1;
    }

    solutions.entry(n).and_modify(|x| *x = length);
    return length;
}

pub struct LongestCollatzSequenceProblem {}

impl Problem for LongestCollatzSequenceProblem {
    fn name(&self) -> String {
        String::from("Longest Collatz Sequence")
    }

    fn number(&self) -> u16 {
        14
    }

    fn solve(&self) -> String {
        const LIMIT: u64 = 1_000_000;

        let mut solutions: HashMap<u64, u64> = HashMap::from([(1, 1)]);
        let mut longest_path = 1;
        let mut longest_path_num = 1;

        let mut i = LIMIT / 2;
        while i < LIMIT {
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
