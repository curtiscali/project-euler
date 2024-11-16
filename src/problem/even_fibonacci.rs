use super::Problem;

pub struct EvenFibonacciProblem {}

impl Problem for EvenFibonacciProblem {
    fn name(&self) -> String {
        String::from("Even Fibonacci Numbers")
    }

    fn number(&self) -> u16 {
        2
    }

    fn solve(&self) -> String {
        const LIMIT: u32 = 4_000_000;

        let mut sum = 0;
        let mut fib1 = 0;
        let mut fib2 = 1;

        while fib2 <= LIMIT {
            let new_fib = fib1 + fib2;
            fib1 = fib2;
            fib2 = new_fib;

            if new_fib % 2 == 0 {
                sum = sum + new_fib;
            }
        }

        return format!("{}", sum);
    }
}
