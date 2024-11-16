use crate::{
    number_theory::{
        from_digits, 
        num_digits, 
        to_digits
    }, 
    primes::sieve_of_atkin
};
use super::Problem;

fn get_rotations(n: usize) -> Vec<usize> {
    let mut rotations: Vec<usize> = Vec::new();
    
    let num_digits = num_digits(n);
    let mut digits = to_digits(n);
    
    let mut i = 0;
    while i < num_digits {
        let d = &mut digits;

        let first = d[0];
        d.remove(0);
        d.push(first);

        rotations.push(from_digits(&d.to_vec()));

        i += 1;
    }

    return rotations;
}

pub struct CircularPrimesProblem {}

impl Problem for CircularPrimesProblem {
    fn name(&self) -> String {
        String::from("Circular Primes")
    }

    fn number(&self) -> u16 {
        35
    }

    fn solve(&self) -> String {
        let primes_below_bound_lookup = sieve_of_atkin(1_000_000);

        let mut circular_primes_count = 0;
        for n in 2..primes_below_bound_lookup.len() {
            if primes_below_bound_lookup[n] {
                let rotations = get_rotations(n);
                let mut are_all_rotations_prime = true;
                for x in rotations {
                    if !primes_below_bound_lookup[x] {
                        are_all_rotations_prime = false;
                        break;
                    }
                }

                if are_all_rotations_prime {
                    circular_primes_count += 1;
                }
            }
        }

        return format!("{}", circular_primes_count);
    }
}
