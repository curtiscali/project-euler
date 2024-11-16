use crate::number_theory::is_palindrome;
use super::Problem;

pub struct LargestPalindromeProduct {}

impl Problem for LargestPalindromeProduct {
    fn name(&self) -> String {
        String::from("Largest Palindrome Product")
    }

    fn number(&self) -> u16 {
        4
    }

    fn solve(&self) -> String {
        let mut largest_palindrome = 0;
        let mut products: (u32, u32) = (0, 0);

        // We can start at half the limit, since the largest palindrome is likely to be
        // towards the top of the range rather than the bottom. This allows us to cut our problem set in half
        let mut i = 1000;
        while i >= 100 {
            let mut j = if i % 11 == 0 {
                999
            } else {
                990
            };

            let delta_j = if i % 11 == 0 {
                1
            } else {
                11
            };

            while j >= i {
                if i * j <= largest_palindrome {
                    break;
                }

                if is_palindrome(i * j) {
                    products = (i, j);
                    largest_palindrome = i * j;
                }

                j -= delta_j;
            }

            i -= 1;
        }

        return format!("{} * {} = {}", products.0, products.1, largest_palindrome);
    }
}
