use std::collections::BTreeSet;
use crate::number_theory::to_digits;
use super::Problem;

fn is_product_pandigital(a: u32, b: u32) -> bool {
    let digits = vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9];
    
    let product = a * b;
    let mut a_digits = to_digits(a);
    let mut b_digits = to_digits(b);
    let mut product_digits = to_digits(product);

    let mut all_digits = vec![];
    all_digits.append(&mut a_digits);
    all_digits.append(&mut b_digits);
    all_digits.append(&mut product_digits);
    all_digits.sort();

    all_digits == digits
}

pub struct PandigitalProductsProblem {}

impl Problem for PandigitalProductsProblem {
    fn solve(&self) -> String {
        let mut discovered_products: BTreeSet<u32> = BTreeSet::new();

        for i in 10..100 {
            for j in 100..1000 {
                let product = i * j;
                if is_product_pandigital(i, j) && !discovered_products.contains(&product) {
                    discovered_products.insert(product);
                }
            }
        }

        for i in 1..10 {
            for j in 1000..10_000 {
                let product = i * j;
                if is_product_pandigital(i, j) && !discovered_products.contains(&product) {
                    discovered_products.insert(product);
                }
            }
        }

        format!("{}", discovered_products.into_iter().sum::<u32>())
    }
}
