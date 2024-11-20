use crate::primes::primes_below;
use super::Problem;

pub struct TotientMaximumProblem {}

impl Problem for TotientMaximumProblem {
    fn name(&self) -> String {
        String::from("Totient Maximum")
    }

    fn number(&self) -> u16 {
        69
    }

    fn solve(&self) -> String {
        const N: usize = 6; // upper bound is 10^N where n = 6, so 1,000,000

        // To achieve a maximal ratio of n/totient(n), we want a highly divisble number,
        // we also want the product of a certain consecutive series of primes
        // Using the asymptotic bounds nln(n)+n(ln(ln(n))−1) < pn < nln(n)+nln(ln(n)) for n >= 6
        // We can generate a list of primes below that upper bound for the nth primes
        // then look at index n - 1 to find our target prime
        let float_n = (N + 1) as f64;
        let ln = float_n.ln();
        let float_upper_bound = (float_n * ln) + (float_n * ln.ln());
        let upper_bound = float_upper_bound as usize;

        let primes = primes_below(upper_bound);
        let max_totient_n = primes.into_iter().product::<u64>();

        return format!("{}", max_totient_n);
    }
}
