use crate::primes::primes_below;
use super::Problem;

pub struct NthPrimeProblem {
    pub n: u32
}

impl Problem for NthPrimeProblem {
    fn solve(&self) -> String {
        // Using the asymptotic bounds nln(n)+n(ln(ln(n))−1) < pn < nln(n)+nln(ln(n)) for n >= 6
        // We can generate a list of primes below that upper bound for the nth primes
        // then look at index n - 1 to find our target prime
        let float_n = self.n as f64;
        let ln = float_n.ln();
        let float_upper_bound = (float_n * ln) + (float_n * ln.ln());
        let upper_bound = float_upper_bound as usize;

        let primes = primes_below::<u32>(upper_bound);

        return format!("{}", primes[(self.n - 1) as usize]);
    }
}
