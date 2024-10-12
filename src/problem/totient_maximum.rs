use crate::primes::sieve_of_eratosthenes;
use crate::arithmetic::totient;
use super::Problem;

pub struct TotientMaximumProblem {
    pub upper_bound: u64
}

impl Problem for TotientMaximumProblem {
    fn solve(&self) -> String {
        let primes_below_bound = sieve_of_eratosthenes(self.upper_bound as usize + 1);

        let mut max_totient_n = 2;
        let mut max_totient_ratio = 0.0;

        let mut i = 0;
        while i < primes_below_bound.len() {
            if !primes_below_bound[i] { // prime numbers will have low totients, so skip that
                let n = (i as u64) + 2;
                
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
