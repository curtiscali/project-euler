use num::integer::Roots;
use super::Problem;

fn matches(n: &u128) -> bool {
    let mut n = *n;

    if n & 10 != 0 {
        return false;
    }

    n /= 100;

    for d in (0..=9).rev() {
        let digit = n % 10;
        if d != digit {
            return false;
        }

        n /= 100;
    }

    true
}

pub struct ConcealedSquaresProblem {}

impl Problem for ConcealedSquaresProblem {
    fn solve(&self) -> String {
        // max = sqrt(1929394959697989990)
        // min = sqrt(1020304050607080900)

        let min = 1020304050607080900u128.sqrt();
        let max = 1929394959697989990u128.sqrt() + 1;

        let mut found = 0u128;
        for i in min..max {
            let x = i * i;

            if matches(&x) {
                found = i;
                break;
            }
        }
        
        format!("{}", found)
    }
}
