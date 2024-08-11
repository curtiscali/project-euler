use num::pow;

use super::Problem;
use crate::primes::primes_below;

fn eval(arr: &[u8; 7]) -> usize {
    let mut number = 0;
    let mut i = 0;
    while i < arr.len() {
        number += (arr[i] as usize) * pow(10, arr.len() - i - 1);
        i += 1;
    }

    return number;
}

fn swap(arr: &mut [u8; 7], i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

// Algorithm based on: https://www.baeldung.com/cs/array-generate-all-permutations#bd-permutations-in-lexicographic-order
fn get_next_permutation(arr: &mut [u8; 7]) -> bool {
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }

    if i <= 0 {
        return false;
    }

    let mut j = arr.len();
    while j > i && arr[j - 1] <= arr[i - 1] {
        j -= 1;
    }

    swap(arr, i -1, j - 1);
    
    i = i + 1;
    j = arr.len();
    while i < j {
        swap(arr, i -1, j - 1);
        i += 1;
        j -= 1;
    }
 
    return true;
}

pub struct PandigitalPrimeProblem {}

impl Problem for PandigitalPrimeProblem {
    fn solve(&self) -> String {
        let primes = primes_below(7_654_322);
        let mut pandigital: [u8; 7] = [7, 1, 2, 3, 4, 5, 6];

        let mut largest_pandigital_prime = 0;
        let mut has_permutations_left = true;
        while has_permutations_left {
            let n = eval(&pandigital);
            
            if primes[n - 2] && n > largest_pandigital_prime {
                largest_pandigital_prime = n;
            }

            has_permutations_left = get_next_permutation(&mut pandigital);
        }

        return format!("{}", largest_pandigital_prime);
    }
}