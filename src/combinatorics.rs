use std::clone;

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

pub fn combinations<T: Num + Clone + PartialOrd>(n: T, r: T) -> T {
    if n < r {
        panic!("n must be less than r");
    } else if n == r || r == T::zero() {
        return T::one();
    }

    let max = if r.clone() > (n.clone() - r.clone()) {
        r.clone()
    } else {
        n.clone() - r.clone()
    };


    let min = if r.clone() < (n.clone() - r.clone()) {
        r
    } else {
        n.clone() - r.clone()
    };

    let numerator = combinatoric_multiplier(max.clone() + T::one(), n.clone());
    let denominator = combinatoric_multiplier(T::one(), min.clone());

    return numerator / denominator;
}
