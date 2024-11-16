use crate::primes::spf_sieve;
use super::Problem;

fn distinct_prime_factors_product(n: u32, smallest_prime_factors: &Vec<u32>) -> u32 {
    let mut factors_product = 1;

    let mut most_recent_factor = 0;
    let mut x = n;
    while x != 1 {
        let x_usize = x as usize;
        let factor = smallest_prime_factors[x_usize];
        if factor != most_recent_factor {
            factors_product *= factor;
            most_recent_factor = factor;
        }

        x /= factor;
    }

    factors_product
}

pub struct OrderedRadicalsProblem {}

impl Problem for OrderedRadicalsProblem {
    fn name(&self) -> String {
        String::from("Ordered Radicals")
    }

    fn number(&self) -> u16 {
        124
    }

    fn solve(&self) -> String {
        let smallest_prime_factors = spf_sieve::<u32>(100_000);
        let mut rad_data: Vec<(u32, u32)> = vec![];

        for n in 1..=100_000 {
            rad_data.push(
                (n, distinct_prime_factors_product(n, &smallest_prime_factors))
            );
        }

        rad_data.sort_by(|(_, rad_a), (_, rad_b)| rad_a.cmp(rad_b));
        let (n, _) = &rad_data[9999]; // 0-based indexing means the 10kth element is 9999

        format!("{}", n)
    }
}
