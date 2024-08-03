use crate::primes::primes_below;
use crate::Problem;

pub struct NthPrimeProblem {
    pub n: u32
}

impl Problem for NthPrimeProblem {
    fn solve(&self) -> String {
        // Using the asymptotic bounds nln(n)+n(ln(ln)n))−1) < pn < nln(n)+nln(ln(n)) for n >= 6
        // This allows us to compute primes using the sieve of eratosthenes instead of brute-forcing
        // by testing each n for primality, which becomes more and more expensive as the numbers grow
        let float_n = self.n as f64;
        let ln = float_n.ln();
        let float_upper_bound = (float_n * ln) + (float_n * ln.ln());
        let upper_bound = float_upper_bound as usize;

        let primes = primes_below(upper_bound);

        let mut nth_prime = 2;
        let mut prime_count = 0;
        let mut i = 0;
        while i < primes.len() {
            if primes[i] {
                nth_prime = i + 2;
                prime_count = prime_count + 1;

                if prime_count >= self.n {
                    break;
                }
            }

            i = i + 1;
        }
        
        return format!("{}", nth_prime);
    }
}