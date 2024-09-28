use num::{BigInt, One};

pub fn mod_tetr(b: u64, p: u64, m: u64) -> BigInt {
    let mut product = BigInt::one();

    if p == 0 {
        return product;
    }

    product = BigInt::from(b);

    let power = BigInt::from(b);
    let modulus = BigInt::from(m);
    
    let mut h = p;
    while h > 1 {
        product = power.modpow(&product, &modulus);//product.modpow(&power, &modulus);
        h -= 1;
    }

    product
}
