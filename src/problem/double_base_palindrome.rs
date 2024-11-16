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
    fn name(&self) -> String {
        String::from("Double-base Palindromes")
    }

    fn number(&self) -> u16 {
        36
    }

    fn solve(&self) -> String {
        return format!("{}", (1..1_000_000).filter(|i| is_palindrome(*i, 10) && is_palindrome(*i, 2)).sum::<u32>());
    }
}
