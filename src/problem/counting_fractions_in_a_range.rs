use std::collections::BTreeSet;

use super::Problem;
use crate::{arithmetic::factors, fractions::Fraction};

pub struct CountingFractionsInARangeProblem {
    pub denom_limit: usize
}

impl Problem for CountingFractionsInARangeProblem {
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

        let mut discovered_fractions: BTreeSet<Fraction> = BTreeSet::new();
        let mut d = denoms_to_examine.len() - 1;
        while denoms_to_examine[d] {
            let mut n = d - 1;
            while n > 0 {
                let reduced_frac = Fraction::reduced(n as u32, d as u32);

                if !discovered_fractions.contains(&reduced_frac) {
                    discovered_fractions.insert(reduced_frac);
                }

                n -= 1;
            }

            d -= 1;
        }

        let one_third = Fraction::new(1, 3);
        let one_half = Fraction::new(1, 2);

        let ordered_fractions_list: Vec<Fraction> = discovered_fractions.into_iter().collect();

        let one_third_index = ordered_fractions_list.binary_search(&one_third).ok().unwrap();
        let one_half_index = ordered_fractions_list.binary_search(&one_half).ok().unwrap();

        return format!("{}", one_half_index - one_third_index - 1);
    }
}
