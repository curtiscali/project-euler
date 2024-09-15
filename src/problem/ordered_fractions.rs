use crate::arithmetic::{factors, gcd};
use super::Problem;
use std::{cmp::Ordering, collections::BTreeSet, fmt::{format, Display}};

#[derive(PartialOrd, Copy, Clone)]
struct Fraction {
    pub numerator: u32,
    pub denominator: u32
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

pub struct OrderedFractionsProblem {
    pub denom_limit: usize
}

impl Problem for OrderedFractionsProblem {
    fn solve(&self) -> String {
        let mut denoms_to_examine = vec![true; self.denom_limit + 1];
        denoms_to_examine[0] = false;
        denoms_to_examine[1] = false;

        let mut i = denoms_to_examine.len() - 1;
        while i > 1 {
            if denoms_to_examine[i] {
                for factor in factors(i) {
                    denoms_to_examine[factor] = false;
                }
            }


            i -= 1;
        }

        let mut n = denoms_to_examine.len() - 1;
        while denoms_to_examine[n] {
            n -= 1;
        }

        return format!("{}", denoms_to_examine.into_iter().filter(|b| *b).count());
    }
}
