use crate::primes::primes_below;
use super::Problem;

// Tis function based on the fourier transform: https://cp-algorithms.com/algebra/phi-function.html#etf_1_to_n
fn totient(n: usize) -> usize {
    let mut totient = n;
    let mut i = 2;

    let mut x = n;
    while i * i < x {
        if x % i  == 0 {
            while x % i == 0 {
                x /= i;
            }

            totient -= totient / i;
        }

        i += 1;
    }

    if x > 1 {
        totient -= totient / x;
    }

    return totient;
}

pub struct TotientMaximumProblem {
    pub upper_bound: usize
}

impl Problem for TotientMaximumProblem {
    fn solve(&self) -> String {
        let primes_below_bound = primes_below(self.upper_bound + 1);

        let mut max_totient_n = 2;
        let mut max_totient_ratio = 0.0;

        let mut i = 0;
        while i < primes_below_bound.len() {
            if !primes_below_bound[i] { // prime numbers will have low totients, so skip that
                let n = i + 2;
                
                let totient = totient(n);

                let totient_ratio = n as f64 / totient as f64;
                if totient_ratio > max_totient_ratio {
                    max_totient_ratio = totient_ratio;
                    max_totient_n = n;
                }
            }
            
            i += 1; 
        }

        return format!("{}", max_totient_n);
    }
}