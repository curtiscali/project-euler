use std::cmp::{max, min};
use num::Num;

fn combinatoric_multiplier<T: Num + Clone + PartialOrd>(n: T, r: T) -> T {
    if n == r {
        return n;
    }

    let mut multiplier = T::one();
    let mut i = n;
    while &i <= &r {
        multiplier = multiplier * i.clone();
        i = i + T::one();
    }

    return multiplier;
}

pub fn combinations<T: Num + Clone + Ord>(n: T, r: T) -> T {
    if n < r {
        panic!("n must be less than r");
    } else if n == r || r == T::zero() {
        return T::one();
    }

    let max = max(r.clone(), n.clone() - r.clone());
    let min = min(r.clone(), n.clone() - r.clone());

    let numerator = combinatoric_multiplier(max.clone() + T::one(), n.clone());
    let denominator = combinatoric_multiplier(T::one(), min.clone());

    return numerator / denominator;
}

pub fn partitions_count(target: u64, partitions: &Vec<u64>) -> u64 {
    let n = target as usize;

    let mut solutions: Vec<u64> = vec![0; n + 1];
    solutions[0] = 1;

    for k in partitions {
        let k_idx = *k as usize;
        for i in 0..=(n - k_idx) {
            solutions[i + k_idx] = solutions[i + k_idx] + solutions[i];
        }
    }

    solutions[n]
}
