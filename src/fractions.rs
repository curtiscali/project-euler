use std::{cmp::Ordering, fmt::Display};
use crate::arithmetic::{factors, gcd};

#[derive(PartialOrd, Copy, Clone)]
pub struct Fraction {
    pub numerator: u32,
    pub denominator: u32
}

impl Fraction {
    pub fn new(n: u32, d: u32) -> Fraction {
        Fraction { numerator: n, denominator: d }
    }

    pub fn reduced(n: u32, d: u32) -> Fraction {
        let gcd = gcd(n, d);
        Fraction::new(n / gcd, d / gcd)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

impl Eq for Fraction {}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        
        if self.eq(other) {
            Ordering::Equal
        } else {
            let this_as_f64 = (self.numerator as f64) / (self.denominator as f64);
            let other_as_f64 = (other.numerator as f64) / (other.denominator as f64);

            if this_as_f64 > other_as_f64 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / {})", self.numerator, self.denominator)    
    }
}
