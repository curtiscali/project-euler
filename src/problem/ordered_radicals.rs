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

struct Radical {
    pub n: u32,
    pub rad_n: u32
}

pub struct OrderedRadicalsProblem {}

impl Problem for OrderedRadicalsProblem {
    fn solve(&self) -> String {
        let smallest_prime_factors = spf_sieve::<u32>(100_000);
        let mut rad_data: Vec<Radical> = vec![];

        for n in 1..=100_000 {
            rad_data.push(Radical {
                n,
                rad_n: distinct_prime_factors_product(n, &smallest_prime_factors)
            });
        }

        rad_data.sort_by(|a, b| a.rad_n.cmp(&b.rad_n));
        let ten_kth_elem = &rad_data[9999]; // 0-based indexing means the 10kth element is 9999

        format!("{}", ten_kth_elem.n)
    }
}
