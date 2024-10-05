use crate::primes::sieve_of_atkin;

use super::Problem;


pub struct GoldbachsOtherConjectureProblem {}

impl Problem for GoldbachsOtherConjectureProblem {
    fn solve(&self) -> String {
        let primes_lookup = sieve_of_atkin(500_000);

        let mut n = 0;
        let mut i = 9; // smallest odd composite number
        while i <= 500_000 {
            let mut refute_conjecture = true;
            if !primes_lookup[i] {
                let mut j = 1;
                while (2 * j * j) < i {
                    let check = i - (2 * j * j);

                    if primes_lookup[check] {
                        refute_conjecture = false;
                        break;
                    }

                    j += 1; 
                }

                if refute_conjecture {
                    n = i;
                    break;
                }
            }

            i += 2;
        }

        format!("{}", n)
    }
}
