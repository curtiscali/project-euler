use crate::{number_theory::to_digits, primes::sieve_of_atkin};
use super::Problem;

pub struct PrimePermutationsProblem {}

impl Problem for PrimePermutationsProblem {
    fn name(&self) -> String {
        String::from("Prime Permutations")
    }

    fn number(&self) -> u16 {
        49
    }

    fn solve(&self) -> String {
        let all_primes_under_10k = sieve_of_atkin(9999);

        let (mut prime_1, mut prime_2, mut prime_3) = (0usize, 0usize, 0usize);

        for i in 1488..all_primes_under_10k.len() {
            if all_primes_under_10k[i] {
                let j = i + 3330;
                let k = i + 6660;

                let mut i_digits = to_digits(i);
                i_digits.sort();

                let mut j_digits = to_digits(j);
                j_digits.sort();

                let mut k_digits = to_digits(k);
                k_digits.sort();

                let are_numbers_permutations = i_digits == j_digits && i_digits == k_digits;

                if k < 10_000 && all_primes_under_10k[j] && all_primes_under_10k[k] && are_numbers_permutations {
                    prime_1 = i;
                    prime_2 = j;
                    prime_3 = k;
                    break;
                }
            }
        }

        format!("{}{}{}", prime_1, prime_2, prime_3)
    }
}
