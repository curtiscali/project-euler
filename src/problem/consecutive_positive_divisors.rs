use std::collections::HashMap;
use super::Problem;

fn spf_sieve(n: usize) -> Vec<usize> {
    let mut smallest_prime_factors = vec![1; n + 1];

    let mut i = 2;
    while i <= n {
        if smallest_prime_factors[i] == 1 {
            let mut j = i;
            while j <= n {
                if smallest_prime_factors[j] == 1 {
                    smallest_prime_factors[j] = i;
                }

                j += i;
            }
        }

        i += 1;
    }

    return smallest_prime_factors;
}

fn sigma_0(n: usize, smallest_prime_factors: &Vec<usize>) -> usize {
    let mut prime_factors: HashMap<usize, usize> = HashMap::new();

    let mut x = n;
    while x != 1 {
        let p = smallest_prime_factors[x];

        prime_factors.entry(p)
            .and_modify(|k| *k = *k + 1)
            .or_insert(1);

        x /= p;
    }

    prime_factors.into_iter().map(|(_, k)| k + 1).product::<usize>()
}

pub struct ConsecutivePositiveDivisorsProblem {
    pub upper_bound: u32
}

impl Problem for ConsecutivePositiveDivisorsProblem {
    fn solve(&self) -> String {
        let smallest_prime_factors = spf_sieve(self.upper_bound as usize);

        let mut num_consecutive = 0u32;
        let mut n = 2;
        while n < self.upper_bound {
            if sigma_0(n as usize, &smallest_prime_factors) == sigma_0((n + 1) as usize, &smallest_prime_factors) {
                num_consecutive += 1;
            }

            n += 1;
        }

        format!("{}", num_consecutive)
    }
}
