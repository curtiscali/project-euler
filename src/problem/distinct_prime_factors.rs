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

fn distinct_prime_factors_count(n: usize, smallest_prime_factors: &Vec<usize>) -> usize {
    let mut factors_count = 0;

    let mut most_recent_factor = 0;
    let mut x = n;
    while x != 1 {
        if smallest_prime_factors[x] != most_recent_factor {
            factors_count += 1;
            most_recent_factor = smallest_prime_factors[x];
        }

        x /= smallest_prime_factors[x];
    }

    factors_count
}

pub struct DistinctPrimeFactorsProblem {}

impl Problem for DistinctPrimeFactorsProblem {
    fn solve(&self) -> String {
        const TARGET_FACTORS_COUNT: usize = 4;

        // 250k is the upper bound since I know the answer will be in the low 6 digits
        let smallest_prime_factors = spf_sieve(250_000);

        let mut found_numbers: Vec<usize> = vec![];
        let mut n = 647;

        while found_numbers.len() < TARGET_FACTORS_COUNT {
            let distinct_prime_factors_count = distinct_prime_factors_count(n, &smallest_prime_factors);

            if distinct_prime_factors_count == TARGET_FACTORS_COUNT {
                if found_numbers.len() == 0 {
                    found_numbers.push(n);
                } else {
                    if n - found_numbers[found_numbers.len() - 1] != 1 {
                        found_numbers.clear();
                    }

                    found_numbers.push(n);
                }
            }

            n += 1;
        }

        format!("{:?}", found_numbers)
    }
}
