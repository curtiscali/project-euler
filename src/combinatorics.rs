use num::Num;

pub fn combinations<T: Num + Clone + PartialOrd>(n: T, r: T) -> T {
    let mut num_combos = T::one();

    let mut i = T::zero();
    while i < r {
        num_combos = num_combos * ((n.clone() - i.clone()) / (i.clone() + T::one()));
        i = i + T::one();
    }

    return num_combos;
}
