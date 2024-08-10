use std::mem;

const FISR_ACCURACY_LIMIT: usize = 410881;

pub fn linear_sum(n: usize) -> usize {
    return (n * (n + 1)) / 2;
}

pub fn quadratic_sum(n: usize) -> usize {
    return (n * (n + 1) * ((2 * n) + 1)) / 6;
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

pub fn num_digits(n: usize) -> usize {
    let mut digit_count = 0;
    let mut number = n;
    while number > 0 {
        digit_count += 1;
        number /= 10;
    }

    return digit_count;
}