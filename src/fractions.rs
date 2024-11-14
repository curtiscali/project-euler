use std::{cmp::Ordering, fmt::Display, ops::Add};
use num::integer::Roots;

use crate::number_theory::{gcd, lcm};

#[derive(PartialOrd, Copy, Clone, Debug)]
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
            let this_as_f64 = if self.denominator == 0 {
                f64::INFINITY
            } else {
                (self.numerator as f64) / (self.denominator as f64)
            };

            let other_as_f64 = if other.denominator == 0 {
                f64::INFINITY
            } else {
                (other.numerator as f64) / (other.denominator as f64)
            };

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

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let lcm = lcm(&vec![self.denominator, rhs.denominator]);

        let self_factor = lcm / self.denominator;
        let rhs_factor = lcm / rhs.denominator;

        let new_numerator = (self.numerator * self_factor) + (rhs.numerator * rhs_factor);

        Fraction { numerator: new_numerator, denominator: lcm }
    }
}

// This function is used in the farey_length function below
// Implemented using Algorithm S6-S9 as described here:
// https://static-content.springer.com/esm/art%3A10.1038%2Fs41598-021-99545-w/MediaObjects/41598_2021_99545_MOESM1_ESM.pdf
struct FareyLookupTable {
    pub n: u32,
    pub x: Vec<Option<u128>>,
    pub y: Vec<Option<u128>>
}

impl FareyLookupTable {
    fn new(n: u32, d: u32) -> Self {
        FareyLookupTable { 
            n, 
            x: vec![None; d as usize], 
            y: vec![None; ((n / d) + 1) as usize] 
        }
    }

    fn get(&self, k: u32) -> Option<u128> {
        let k_idx = k as usize;
        if k_idx < self.x.len() {
            return self.x[k_idx];
        }

        self.y[(self.n as usize) / k_idx]
    }

    fn contains(&self, k: u32) -> bool {
        let k_idx = k as usize;
        let n_idx = self.n as usize;
        if k_idx < self.x.len() {
            return self.x[k_idx].is_some();
        }

        let m = n_idx / k_idx; 
        if k_idx != (n_idx / m) {
            return false;
        }

        self.y[m].is_some()
    }

    fn set(&mut self, k: u32, v: u128) {
        let k_idx = k as usize;
        if k_idx < self.x.len() {
            self.x[k_idx] = Some(v);
        } else {
            self.y[(self.n as usize) / k_idx] = Some(v);
        }
    }
}

fn update_lookup_table(lookup: &mut FareyLookupTable, m: u32) {
    if m <= 1 {
        return;
    }

    let r = m.sqrt();
    let u = m / (r + 1);
    let mut s = 0;

    for k in 2..=u {
        let q = m / k;
        if lookup.contains(q) {
            s += lookup.get(q).unwrap();
        }
    }

    for k in 1..=r {
        if lookup.contains(k) {
            let (a, b) = (m / k, m / (k + 1));
            s += (a - b) as u128 * lookup.get(k).unwrap();
        }
    }

    let m_u128 = m as u128;
    lookup.set(m, ((m_u128 * (m_u128 + 3)) / 2) - s);
}

// This algorithm is my implementation of Algorithm C with time complexity n^3/4
// https://pmc.ncbi.nlm.nih.gov/articles/PMC8593030/#Sec6
pub fn farey_length(n: u32, include_ends: Option<bool>) -> u128 {
    let r = n.sqrt();
    let u = n / (r + 1);

    let mut f = FareyLookupTable::new(n, r + 1);
    f.set(1, 2);

    for m in 2..=r {
        update_lookup_table(&mut f, m);
    }

    for j in (1..=u).rev() {
        update_lookup_table(&mut f, n / j);
    }

    let num_fractions_to_exclude = match include_ends {
        Some(should_include_ends) => if should_include_ends {
            0
        } else {
            2
        },
        None => 0
    };

    f.get(n).unwrap() - num_fractions_to_exclude
}
