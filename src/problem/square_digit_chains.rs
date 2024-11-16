use std::collections::HashMap;

use super::Problem;

fn digit_sqare_sum(n: u32) -> u32 {
    let mut x = n;
    let mut digit_square_sum = 0;

    while x > 0 {
        let digit = x % 10;
        digit_square_sum += digit * digit;
        x /= 10;
    }

    return digit_square_sum;
}

fn digit_chain(n: u32, cache: &mut HashMap<u32, bool>) -> bool {
    let mut copy = n;

    while copy != 89 && copy != 1 {
        copy = digit_sqare_sum(copy);

        match cache.get(&copy) {
            Some(is_one_or_89) => {
                cache.insert(n, *is_one_or_89);
                return *cache.get(&n).unwrap();
            },
            None => {}
        }
    }

    cache.insert(n, copy == 89);
    return *cache.get(&n).unwrap();
}

pub struct SquareDigitChainsProblem {}

impl Problem for SquareDigitChainsProblem  {
    fn name(&self) -> String {
        String::from("Square Digit Chains")
    }

    fn number(&self) -> u16 {
        91
    }

    fn solve(&self) -> String {
        let mut chain_cache: HashMap<u32, bool> = HashMap::new();
        let mut num_qualified = 0;

        for i in 1..10_000_000 {
            if digit_chain(i, &mut chain_cache) {
                num_qualified += 1;
            }
        }

        return format!("{}", num_qualified);
    }
}
