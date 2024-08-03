use crate::Problem;

fn triangle(n: u128) -> u128 {
    return (n * (n + 1)) / 2;
}

fn num_factors(n: u128) -> u32 {
    let mut factor_count: u32 = 0;
    let root = (n as f64).sqrt().ceil() as u128;

    let mut i = 1;
    while i <= root {
        if n % i == 0 {
            if i == root {
                factor_count += 1;
            } else {
                factor_count += 2;
            }
        }

        i += 1;
    }

    return factor_count;
}

pub struct HighlyDivisibleTriangleNumber {
    pub num_divisors: u32
}

impl Problem for HighlyDivisibleTriangleNumber {
    fn solve(&self) -> String {
        let mut n = 7;
        while num_factors(triangle(n)) <= self.num_divisors {
            n += 1;
        }

        return format!("The first triangular number with over {} factors is the {}th triangular number ({})", self.num_divisors, n, triangle(n));
    }
}