use std::{collections::HashMap};
use num::{Num, PrimInt};
use crate::number_theory::fast_modpow;

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

pub fn spf_sieve(n: usize) -> Vec<u64> {
    let mut smallest_prime_factors = vec![1; n + 1];

    for i in 2..=n {
        if smallest_prime_factors[i] == 1 {
            for j in (i..=n).step_by(i) {
                if smallest_prime_factors[j] == 1 {
                    smallest_prime_factors[j] = i as u64;
                }
            }
        }
    }

    smallest_prime_factors
}

pub fn prime_factors(number: u64) -> HashMap<u64, u64> {
    let mut factors: HashMap<u64, u64> = HashMap::new();
    let mut n = number;

    while n % 2 == 0 {
        factors.entry(2)
            .and_modify(|x| *x += 1)
            .or_insert(1);

        n /= 2;
    }

    let mut i = 3;
    while i * i <= n {   
        while n % i == 0 {
            factors.entry(i)
                .and_modify(|x| *x += 1)
                .or_insert(1);
            n = n / i;
        }

        i += 2;
    }

    if n > 2 {
        factors.entry(n)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    return factors;
}

pub fn primes_below(n: usize) -> Vec<u64> {
    let sieve_lookup = sieve_of_atkin(n);
    let mut primes = vec![];

    for i in 0..sieve_lookup.len() {
        if sieve_lookup[i] {
            primes.push(i as u64);
        }
    }

    primes
}

pub fn fermat_primality_test<T: Num + Copy + PartialOrd + Eq >(n: T, tests: T) -> bool {
    let two = T::one() + T::one();

    let mut x = T::zero();
    while x < tests {
        if fast_modpow(two * (x + two), n - T::one(), n) != T::one() {
            return false;
        }

        x = x + T::one();
    }

    true
}

fn is_composite(a: u64, d: u64, n: u64, r: u32) -> bool {
    if fast_modpow(a, d, n) == 1 {
        return false;
    }

    for i in 0..r {
        if fast_modpow(a, 2.pow(i) * d, n) == n - 1 {
            return false;
        }
    }

    true
}

pub fn miller_primality_test(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 || n == 3 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let tests = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    for k in tests {
        if is_composite(k, d, n, r) {
            return false;
        }
    }

    true
}

// Tis function based on the fourier transform: https://cp-algorithms.com/algebra/phi-function.html#etf_1_to_n
pub fn totient(n: u64) -> u64 
{
    let prime_factors = prime_factors(n);

    let mut product = n as f64;
    for (p, _) in prime_factors {
        product = product * (1.0 - (1.0 / (p as f64)));
    }

    product as u64
}
