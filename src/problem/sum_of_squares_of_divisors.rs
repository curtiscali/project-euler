use crate::primes::{prime_factors, sieve_of_atkin};

use super::Problem;

fn sum_1(n: u128) -> u128 {
    let n_sqrt = (n as f64).sqrt().floor() as u128;
    let mut sum = 0;

    let mut x = 1;
    while x <= n_sqrt {
        sum += n * n;

        let mut y = 1;
        while y <= (n / x) {
            sum += 1;
            y += 1;
        }

        x += 1;
    }
    
    sum
}

fn sum_2(n: u128) -> u128 {
    let n_sqrt = (n as f64).sqrt().floor() as u128;
    let mut sum = 0;

    let mut y = 1;
    while y < n_sqrt {
        sum += 1;

        let mut x = 1;
        while x <= (n / y) {
            sum += n * n;
            x += 1;
        }

        y += 1;
    }

    sum
}

fn sum_3(n: u128) -> u128 {
    let n_sqrt = (n as f64).sqrt().floor() as u128;
    let mut sum = 0;

    let mut x = 1;
    while x <= n_sqrt {
        sum += n * n;

        let mut y = 1;
        while y <= n_sqrt {
            sum += 1;
            y += 1;
        }

        x += 1;
    }

    sum
}

pub struct SumOfSquaresOfDivisorsProblem {
    pub limit: u128,
    pub divisor: u128
}

impl Problem for SumOfSquaresOfDivisorsProblem {
    fn solve(&self) -> String {
        // Dirichlet Convolution for sigma2(n) = sum(d^2)(d|n)
        // g(x) = x^2
        // h(y) = 1
        // x = y = sqrt(n)

        let sigma2_sum = sum_1(self.limit) + sum_2(self.limit) - sum_3(self.limit);
        format!("{}", sigma2_sum)
    }
}
