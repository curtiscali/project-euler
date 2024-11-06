use std::collections::HashMap;
use num::integer::Roots;
use crate::Problem;

// This function implements the sub-linear algorithm from:
// https://projecteuler.net/thread=10;page=5#111677
// AND
// https://gbroxey.github.io/blog/2023/04/09/lucy-fenwick.html
fn primes_sum(n: u64) -> u64 {
    let sqrt = n.sqrt();
    let mut v = (1..=sqrt).into_iter().map(|x| n / x).collect::<Vec<u64>>();
    let mut s: HashMap<u64, u64> = HashMap::new();

    for x in (1..v[v.len() - 1]).rev() {
        v.push(x);
    }

    for i in &v {
        let val = ((*i * (*i + 1)) / 2) - 1;
        s.insert(*i, val);
    }

    for p in 2..=sqrt {
        let minus_one = p - 1;
        if s[&p] > s[&minus_one] {
            let sp = s[&minus_one];
            let p2 = p * p;
            for x in &v {
                if *x < p2 {
                    break;
                }

                let div_p = x / p;
                let y = s[&div_p];
                s.entry(*x).and_modify(|z| *z = *z - p * (y - sp));
            }
        }
    }

    s[&n]
}

pub struct SummationOfPrimes {}

impl Problem for SummationOfPrimes {
    fn solve(&self) -> String {
        format!("{}", primes_sum(2_000_000))
    }
}
