use super::Problem;

fn digit_factorial_sum(n: u64) -> u64 {
    let factorial_values: [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    let mut digit_factorial_sum: u64 = 0;
    let mut num = n;
    while num > 0 {
        let digit = (num as usize) % 10;
        digit_factorial_sum += factorial_values[digit];
        num /= 10;
    }

    return digit_factorial_sum;
}

pub struct DigitFactorialsProblem {}

impl Problem for DigitFactorialsProblem {
    fn solve(&self) -> String {
        let mut matching_sum = 0;
        for i in 10..=41_000 {
            if i == digit_factorial_sum(i) {
                matching_sum += i;
            }
        }

        return format!("{}", matching_sum);
    }
}
