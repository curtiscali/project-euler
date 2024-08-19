use num::BigInt;

pub fn combinations_bigint(n: BigInt, r: BigInt) -> BigInt {
    let mut num_combos = BigInt::from(1);

    let mut i = BigInt::ZERO;
    while i < r {
        num_combos = num_combos * ((n.clone() - i.clone()) / (i.clone() + BigInt::from(1)));
        i = i + 1;
    }

    return num_combos;
}

pub fn combinations_usize(n: usize, r: usize) -> usize {
    let mut combos = 1;

    let mut i = 0;
    while i < r {
        combos *= (n - i) / (i + 1);
        i += 1;
    }

    return combos;
}