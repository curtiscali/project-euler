use std::mem;
use num::{pow, BigInt, Num, Unsigned};

const EPSILON: f64 = 1E-11;
const FISR_ACCURACY_LIMIT: usize = 410881;

pub fn linear_sum<T: Unsigned + Copy>(n: T) -> T {
    let two = T::one() + T::one();

    return (n * (n + T::one())) / two;
}

pub fn quadratic_sum<T: Unsigned + Copy>(n: T) -> T {
    let two = T::one() + T::one();
    let six = T::one() + T::one() + T::one() + T::one() + T::one() + T::one();

    return (n * (n + T::one()) * ((two * n) + T::one())) / six;
}

pub fn bigint_quadratic_sum(n: &BigInt) -> BigInt {
    (n * (n + 1) * ((2 * n) + 1)) / 6 
}

pub fn f64_equals(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

// Algo stolen from: https://github.com/emkw/rust-fast_inv_sqrt/blob/master/src/lib.rs#L65
// WARNING, use only for N < 410881
pub fn fast_inverse_sqrt(n: f64) -> f64 {
    if n.signum() != 1.0 {
        return f64::NAN;
    } else if n == f64::INFINITY {
        return 0.0;
    } else if n < f64::MIN_POSITIVE {
        return f64::INFINITY;
    }

    // Magic number based on Chris Lomont work:
    const MAGIC_U64: usize = 0x5fe6ec85e7de30da;
    const THREEHALFS: f64 = 1.5;
    let x2 = n * 0.5;
    let i = MAGIC_U64 - ( unsafe { mem::transmute::<_, usize>(n) } >> 1);
    let y: f64 = unsafe { mem::transmute(i) };

    return y * (THREEHALFS - (x2 * y * y));
}

pub fn is_perfect_square(n: usize) -> bool {
    let root: usize;
    if n < FISR_ACCURACY_LIMIT {
        let n_f64 = n as f64;
        root = ((fast_inverse_sqrt(n_f64) * n_f64) + 0.5) as usize;
    } else {
        root = ((n as f64).sqrt() + 0.5) as usize;
    }

    return root * root == n; 
}

pub fn is_triangular(n: usize) -> bool {
    return is_perfect_square((8 * n) + 1);
}

pub fn num_digits<T: Num + Copy + PartialOrd>(n: T) -> T {
    let ten = T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one();

    let mut digit_count = T::zero();
    let mut number = n;
    while number > T::zero() {
        digit_count = digit_count + T::one();
        number = number / ten.clone();
    }

    return digit_count;
}

pub fn bigint_num_digits(b: &BigInt) -> BigInt {
    let mut digit_count = BigInt::ZERO;
    let mut n = b.clone();

    while n > BigInt::ZERO {
        digit_count += 1;
        n /= 10;
    }

    digit_count
}

pub fn to_digits<T: Unsigned + Copy + PartialOrd>(n: T) -> Vec<T> {
    let ten = T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one();

    let mut digits: Vec<T> = Vec::new();

    let mut i = n;
    while i > T::zero() {
        digits.insert(0, i % ten);
        i = i / ten;
    }

    return digits;
}

pub fn from_digits(digits: &Vec<usize>) -> usize {
    let mut number: usize = 0;
    let mut i = 0;
    while i < digits.len() {
        number += (digits[i]) * pow(10, digits.len() - i - 1);
        i += 1;
    }

    return number;
}

pub fn digit_sum<T: Unsigned + Copy + PartialOrd>(number: T) -> T {
    let ten = T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one();

    let mut n = number;
    let mut digit_sum = T::zero();

    while n > T::zero() {
        digit_sum = digit_sum + (n % ten);
        n = n / ten;
    }

    return digit_sum;
}

pub fn bigint_digit_sum(number: &BigInt) -> BigInt {
    let mut digit_sum = BigInt::ZERO;
    let mut n = number.clone();

    while n > BigInt::ZERO {
        digit_sum += &n % 10;
        n /= 10;
    }

    digit_sum
}

pub fn factorial(n: usize, solutions: &mut Vec<BigInt>) -> BigInt {
    if n <= solutions.len() {
        return solutions[n - 1].clone();
    }

    let mut i = solutions.len();
    while i <= n {
        solutions.push(solutions[i - 1].clone() * i);
        i += 1;
    }

    return solutions[n - 1].clone();
}

pub fn reverse<T: Unsigned + Copy + PartialOrd>(n: T) -> T {
    let ten = T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one();


    let mut reversed_number = T::zero();
    let mut i = n;

    while i > T::zero() {
        reversed_number = reversed_number * ten;
        reversed_number = reversed_number + (i % ten);
        i = i / ten;
    }
    
    return reversed_number;

}

pub fn is_palindrome<T: Unsigned + Copy + PartialOrd>(n: T) -> bool {
    n == reverse(n)
}

pub fn gcd<T: Unsigned + Copy>(a: T, b: T) -> T {
    let mut x = a;
    let mut y = b;

    while y != T::zero() {
        let tmp = y.clone();
        y = x % y;
        x = tmp;
    }

    return x;
}

pub fn lcm<T: Unsigned + Copy>(numbers: &Vec<T>) -> T {
    if numbers.len() == 0 {
        panic!("Cannot find the LCM of an empty vec");
    }

    let mut lcm = numbers[0];
    let mut i = 0;
    while i < numbers.len() {
        let n = lcm;
        let m = numbers[i];

        let gcd = gcd(n, m);
        lcm = (lcm * numbers[i]) / gcd;

        i += 1;
    }

    return lcm;
}

pub fn sqrt_usize(n: usize) -> usize {
    ((n as f64).sqrt() + 0.5) as usize
}

// Tis function based on the fourier transform: https://cp-algorithms.com/algebra/phi-function.html#etf_1_to_n
pub fn totient<T: Unsigned + Copy + PartialOrd>(n: T) -> T {
    let mut totient = n;
    let mut i = T::one() + T::one();

    let mut x = n;
    while i * i < x {
        if x % i  == T::zero() {
            while x % i == T::zero() {
                x = x / i;
            }

            totient = totient - (totient / i);
        }

        i = i + T::one();
    }

    if x > T::one() {
        totient = totient - (totient / x);
    }

    return totient;
}

pub fn factors<T: Unsigned + Copy + PartialEq + PartialOrd>(n: T) -> Vec<T> {
    let mut i = T::one() + T::one();
    let mut factors: Vec<T> = vec![];

    while i * i <= n {
        if n % i == T::zero() {
            let factor_1 = i;
            let factor_2 = n / i;

            factors.push(factor_1);
            factors.push(factor_2);
        }

        i = i + T::one();
    }

    return factors;
}

pub fn fast_pow<T: Num + Copy + PartialEq + PartialOrd>(a: T, b: T) -> T {
    let two = T::one() + T::one();

    let mut result = T::one();
    let (mut base, mut power) = (a, b);
    while power > T::zero() {
        if power % two == T::one() {
            result = result * base;
        }

        base = base * base;
        power = power / two;
    }

    result
}

pub fn fast_modpow<T: Num + Copy + PartialEq + PartialOrd>(a: T, b: T, m: T) -> T {
    let two = T::one() + T::one();

    let mut result = T::one();
    let (mut base, mut power) = (a, b);
    while power > T::zero() {
        if power % two == T::one() {
            result = (result * base) % m;
        }

        base = (base * base) % m;
        power = power / two;
    }

    result
}
