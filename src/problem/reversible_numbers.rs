use super::Problem;

pub struct ReversibleNumbersProblem {}

impl Problem for ReversibleNumbersProblem {
    fn name(&self) -> String {
        String::from("Reversible Numbers")
    }

    fn number(&self) -> u16 {
        145
    }

    fn solve(&self) -> String {
        // This algorithm based on numerical analysis from: https://projecteuler.net/thread=145#6507
        // let n = the number of digits for a number
        // single-digit numbers (0-9) cannot be reversible by the definition given inhttps://projecteuler.net/problem=145
        // therefore, we only want to look a numbers with digits between 2 & 9 (all numbers between 10 and less than 1 billion)
        // if n is even (n = 2k) => num reversible = 20 * 30^k-1
        // if n = 4k + 3 => num reversible = 100 * 500^k

        let mut num_reversibles = 0u32;
        for n in 2..=9 {
            if n % 2 == 0 {
                let k = n / 2;
                num_reversibles += 20 * 30u32.pow(k - 1);
            } else if n % 4 == 3 {
                let k = (n - 3) / 4;
                num_reversibles += 100 * 500u32.pow(k);
            }
        }

        format!("{}", num_reversibles)
    }
}
