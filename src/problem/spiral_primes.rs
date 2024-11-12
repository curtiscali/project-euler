use crate::primes::fermat_primality_test;
use super::Problem;

// # of numbers along both diagonals = 2n - 1
fn num_values_in_diagonals(n: u64) -> u64 { (2 * n) - 1 }

pub struct SpiralPrimesProblem {}

impl Problem for SpiralPrimesProblem {
    fn solve(&self) -> String {
        const MAX_SPIRAL_SIZE: u64 = 30_000;
        const NUM_TESTS: u64 = 20;

        let mut current_spiral_dim = 9;
        let mut found_primes = 8u32;

        while current_spiral_dim <= MAX_SPIRAL_SIZE {
            let current_diagonal_count = num_values_in_diagonals(current_spiral_dim);
            let x = current_spiral_dim / 2;

            let right_down = (4 * x * x) + (4 * x) + 1;
            let left_down = (4 * x * x) + (2 * x) + 1;
            let right_up = (4 * x * x) - (2 * x) + 1;
            let left_up = (4 * x * x) + 1;

            if fermat_primality_test(right_down, NUM_TESTS) {
                found_primes += 1;
            }

            if fermat_primality_test(left_down, NUM_TESTS) {
                found_primes += 1;
            }
            
            if fermat_primality_test(right_up, NUM_TESTS) {
                found_primes += 1;
            }
            
            if fermat_primality_test(left_up, NUM_TESTS) {
                found_primes += 1;
            }

            let pi_ratio = (found_primes as f64) / current_diagonal_count as f64;
            if pi_ratio < 0.1 {
                break;
            }

            current_spiral_dim += 2;
        }

        format!("{}", current_spiral_dim)
    }
}
