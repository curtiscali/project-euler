use super::Problem;
use crate::arithmetic::is_palindrome;

fn is_binary_palindrome(n: usize) -> bool {
    let mut binary_digits: Vec<usize> = vec![];
    let mut x = n;

    while x > 0 {
        binary_digits.push(x % 2);
        x /= 2;
    }

    let mut i = 0;
    let mut j = binary_digits.len() - 1;

    while i < j && i < binary_digits.len() {
        if binary_digits[i] != binary_digits[j] {
            return false;
        }

        i += 1;
        j -= 1;
    }

    return true;
}

pub struct DoubleBasePalindromeProblem {
    pub upper_bound: usize
}

impl Problem for DoubleBasePalindromeProblem {
    fn solve(&self) -> String {
        let mut palindrome_sum = 0;
        let mut i: usize = 1;

        while i < self.upper_bound {
            if is_palindrome(i) && is_binary_palindrome(i) {
                palindrome_sum += i;
            }

            i += 1;
        }

        return format!("{}", palindrome_sum);
    }
}
