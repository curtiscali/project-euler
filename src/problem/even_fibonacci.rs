use super::Problem;

pub struct EvenFibonacciProblem {
    pub limit: i32
}

impl Problem for EvenFibonacciProblem {
    fn solve(&self) -> String {
        let mut sum: i32 = 0;
        let mut fib1: i32 = 0;
        let mut fib2: i32 = 1;

        while fib2 <= self.limit {
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