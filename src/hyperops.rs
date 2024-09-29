use num::{BigInt, One};

pub fn mod_tetr(a: &BigInt, b: &BigInt, m: &BigInt) -> BigInt {
    let mut product = BigInt::one();

    if b == &BigInt::ZERO {
        return product;
    }

    product = a.clone();
    let mut h = b.clone();
    while h > BigInt::one() {
        product = a.modpow(&product, m);
        h -= 1;
    }

    product
}

pub fn mod_pent(a: &BigInt, b: &BigInt, m: &BigInt) -> BigInt {
    let mut product = BigInt::one();

    if b == &BigInt::ZERO {
        return product;
    }

    let tetr_result = mod_tetr(a, b, m);
    product = tetr_result.clone();

    let mut h = b.clone();
    while h > BigInt::one() {
        product = mod_tetr(a, &product, m);
        h -= 1;
    }

    product
}
