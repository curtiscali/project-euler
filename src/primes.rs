use std::{collections::HashMap, hash::Hash};

use num::{Num, PrimInt, Unsigned};

pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let vec_len = n - 2;
    let mut primes = vec![true; vec_len];

    let root = (vec_len as f32).sqrt() as usize; 
    let mut i = 0;
    while i < root {
        if primes[i] {
            let mut j = (i + 2) * (i + 2);
            while j < n {
                primes[j - 2] = false;
                j = j + i + 2;
            }
        }

        i = i + 1;
    }

    return primes;
}

pub fn sieve_of_atkin(n: usize) -> Vec<bool> {
    let mut primes = vec![false; n + 1];

    if n > 2 {
        primes[2] = true;
    }

    if n > 3 {
        primes[3] = true;
    }

    let mut x = 1;
    while x * x <= n {
        let mut y = 1;
        while y * y <= n {
            let mut m = (4 * x * x) + (y * y);
            if m <= n && (m % 12 == 1 || m % 12 == 5) {
                primes[m] ^= true;
            }

            m = (3 * x * x) + (y * y);
            if m <= n && m % 12 == 7 {
                primes[m] ^= true;
            }

            if x > y {
                m = (3 * x * x) - (y * y);
                if m <= n && m % 12 == 11 {
                    primes[m] ^= true;
                }
            }

            y += 1;
        }

        x += 1;
    }

    let mut r = 5;
    while r * r <= n {
        if primes[r] {
            let mut i = r * r;
            while i <= n {
                primes[i] = false;
                i += r * r;
            }
        }

        r += 1;
    }

    return primes;
}

pub fn prime_factors<T: Num + Copy + PartialOrd + Hash + Eq>(number: T) -> HashMap<T, T> {
    let two = T::one() + T::one();

    let mut factors: HashMap<T, T> = HashMap::new();
    let mut n = number;

    while n % two == T::zero() {
        factors.entry(two)
            .and_modify(|x| *x = *x + T::one())
            .or_insert(T::one());

        n = n / two;
    }

    let mut i = T::one() + T::one() + T::one();
    while i * i <= n {   
        while n % i == T::zero() {
            factors.entry(i)
                .and_modify(|x| *x = *x + T::one())
                .or_insert(T::one());
            n = n / i;
        }

        i = i + two;
    }

    if n > two {
        factors.entry(n)
            .and_modify(|x| *x = *x + T::one())
            .or_insert(T::one());
    }

    return factors;
}

pub fn primes_below<T: Unsigned + PrimInt>(n: usize) -> Vec<T> {
    let sieve_lookup = sieve_of_atkin(n);
    let mut primes: Vec<T> = vec![];

    for i in 0..sieve_lookup.len() {
        if sieve_lookup[i] {
            primes.push(T::from(i).unwrap());
        }
    }

    primes
}
