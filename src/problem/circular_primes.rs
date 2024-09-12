use crate::{
    arithmetic::{
        from_digits, 
        num_digits, 
        to_digits
    }, 
    primes::primes_below
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

        rotations.push(from_digits(d.to_vec()));

        i += 1;
    }

    return rotations;
}

pub struct CircularPrimesProblem {
    pub upper_bound: u32
}

impl Problem for CircularPrimesProblem {
    fn solve(&self) -> String {
        let primes_below_bound = primes_below(self.upper_bound as usize);

        let mut circular_primes_count = 0;
        let mut i = 0; // 13 primes below 100, 98 = 100 - 2
        while i < primes_below_bound.len() {
            let n = i + 2;
            if primes_below_bound[i] {
                let rotations = get_rotations(n);
                let mut are_all_rotations_prime = true;
                for x in rotations {
                    if !primes_below_bound[x - 2] {
                        are_all_rotations_prime = false;
                        break;
                    }
                }

                if are_all_rotations_prime {
                    circular_primes_count += 1;
                }
            }

            i += 1;
        }

        return format!("{}", circular_primes_count);
    }
}
