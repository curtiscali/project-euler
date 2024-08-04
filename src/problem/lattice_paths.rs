use num::ToPrimitive;

use crate::Problem;

fn combo(n: u64, k: u64) -> u64 {
    let mut combos: f64 = 1.0;

    let mut i = 0;
    while i < k {
        combos *= (n - i).to_f64().unwrap() / (i + 1).to_f64().unwrap();
        i += 1;
    }

    return combos.to_u64().unwrap();
}

pub struct LatticePathsProblem { }

impl Problem for LatticePathsProblem {
    fn solve(&self) -> String {
        return format!("{}", combo(40, 20));
    }
}