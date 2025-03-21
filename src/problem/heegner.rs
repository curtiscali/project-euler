use crate::number_theory::is_perfect_square;
use rug::{float::Constant::Pi, ops::CompleteRound, Float};

use super::BonusProblem;

const PRECISION: u32 = 53;

fn cosh(x: &Float) -> Float {
    let two = Float::with_val(PRECISION, 2);

    let a = x.exp_ref();
    
    let neg_x = x.as_neg();
    let b = neg_x.exp_ref();

    (Float::with_val(PRECISION, a) + Float::with_val(PRECISION, b)) / two
}

fn min(a: &Float, b: &Float) -> Float {
    let epsilon = Float::with_val(PRECISION, 1e-11);

    if Float::with_val(PRECISION, a - b) > epsilon {
        a.clone()
    } else {
        b.clone()
    }
}

pub struct HeegnerProblem {}

impl BonusProblem for HeegnerProblem {
    fn name(&self) -> String {
        String::from("Heegner")
    }

    fn solve(&self) -> String {
        let pi = Float::with_val(PRECISION, Pi);

        let mut min_int_diff = f64::MAX;
        let mut min_n = 0;

        for n in 2..=1000 {
            if !is_perfect_square(n) {                
                let x = &pi * Float::with_val(PRECISION, n);

                let cos = x.cos();
                let cosh = x.cosh();

                // let t1 = min(cos - cos.floor(), cos.ceil() - cos);
                // let t2 = min(cosh - cosh.floor(), cosh.ceil() - cosh);
                
                // if t1 < min_int_diff {
                //     min_int_diff = t1;
                //     min_n = n;
                // }

                // if t2 < min_int_diff {
                //     min_int_diff = t2;
                //     min_n = n;
                // }
            }
        }

        format!("{}", min_n)
    }
}
