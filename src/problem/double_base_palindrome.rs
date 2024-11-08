use super::Problem;

fn reverse(n: u32, base: u32) -> u32 {
    let (mut reversed, mut i) = (0, n);

    while i > 0 {
        reversed *= base;
        reversed += i % base;
        i /= base;
    }

    reversed
}

fn is_palindrome(n: u32, base: u32) -> bool {
    n == reverse(n, base)
}

pub struct DoubleBasePalindromeProblem {}

impl Problem for DoubleBasePalindromeProblem {
    fn solve(&self) -> String {
        let mut palindrome_sum = 0;

        for i in 1..1_000_000 {
            if is_palindrome(i, 10) && is_palindrome(i, 2) {
                palindrome_sum += i;
            }
        }

        return format!("{}", palindrome_sum);
    }
}
