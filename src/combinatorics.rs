use std::cmp::{max, min};
use num::{BigInt, Num, One};

fn combinatoric_multiplier<T: Num + Copy + PartialOrd>(n: T, r: T) -> T {
    if n == r {
        return n;
    }

    let mut multiplier = T::one();
    let mut i = n;
    while i <= r {
        multiplier = multiplier * i;
        i = i + T::one();
    }

    return multiplier;
}

fn bigint_combinatoric_multiplier(n: &BigInt, r: &BigInt) -> BigInt {
    if n == r {
        return n.clone();
    }

    let mut multiplier = BigInt::one();
    let mut i = n.clone();
    while &i <= r {
        multiplier *= &i;
        i = i.checked_add(&BigInt::one()).unwrap();
    }

    multiplier
}

pub fn combinations<T: Num + Copy + Ord>(n: T, r: T) -> T {
    if n < r {
        panic!("n must be less than r");
    } else if n == r || r == T::zero() {
        return T::one();
    }

    let max = max(r, n - r);
    let min = min(r, n - r);

    let numerator = combinatoric_multiplier(max + T::one(), n);
    let denominator = combinatoric_multiplier(T::one(), min);

    return numerator / denominator;
}

pub fn bigint_combinations(n: &BigInt, r: &BigInt) -> BigInt {
    if n < r {
        panic!("n must be less than r");
    } else if n == r || r == &BigInt::ZERO {
        return BigInt::one();
    }

    let max = max(r.clone(), n - r);
    let min = min(r.clone(), n - r);

    let numerator = bigint_combinatoric_multiplier(&(max + BigInt::one()), n);
    let denominator = bigint_combinatoric_multiplier(&BigInt::one(), &min);

    numerator / denominator
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
