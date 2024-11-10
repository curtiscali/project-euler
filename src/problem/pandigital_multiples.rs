use crate::number_theory::{fast_pow, from_digits, to_digits};
use super::Problem;

pub struct PandigitalMultiplesProblem {}

impl Problem for PandigitalMultiplesProblem {
    fn solve(&self) -> String {
        let digits = vec![1usize, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut max_pandigital = 0;

        for z in 2usize..=9 {
            for x in 1usize..fast_pow(10, 9 / z) {
                let mut all_digits: Vec<usize> = vec![];
                for y in 1..=z {
                    let product = x * y;
                    let product_digits = to_digits(product);

                    all_digits.extend(product_digits);

                    let mut copy = all_digits.clone();
                    copy.sort();

                    if copy == digits {
                        let num = from_digits(&all_digits);
                        if num > max_pandigital {
                            max_pandigital = num;
                        }
                    }
                }
            }
        }

        format!("{}", max_pandigital)
    }
}
