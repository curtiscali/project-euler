use num::BigInt;

pub fn combinations_bigint(n: BigInt, r: BigInt) -> BigInt {
    let mut y = r.clone();

    if r > (n.clone() - r.clone()) {
        y = n.clone() - r.clone();
    }

    let mut num_combos = BigInt::from(1);
    let mut i = BigInt::ZERO;
    while i.clone() < y.clone() - BigInt::from(1) {
        num_combos = (num_combos * (n.clone() - i.clone())) / (i.clone() + BigInt::from(1));
        i += BigInt::from(1);   
    }

    return num_combos;
}

pub fn combinations_usize(n: usize, r: usize) -> usize {
    let mut y = r;
    if r > n - r {
        y = n - r;
    }

    let mut num_combos = 1;
    let mut i = 0;
    while i < y - 1 {
        num_combos = (num_combos * (n - i)) / (i + 1);
        i += 1;
    }

    return num_combos;
}