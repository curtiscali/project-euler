use crate::arithmetic::{factors, gcd};
use super::Problem;
use crate::fractions::Fraction;

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

        let mut discovered_fractions: Vec<Fraction> = vec![];
        let mut d = denoms_to_examine.len() - 1;
        while denoms_to_examine[d] {
            let mut n = d - 1;
            while n > 0 {
                let reduced_frac = Fraction::reduced(n as u32, d as u32);

                match discovered_fractions.binary_search(&reduced_frac) {
                    Ok(_) => {}, // if we've added it we don't need to do anything
                    Err(insert_index) => {
                        discovered_fractions.insert(insert_index, reduced_frac);
                    }
                }

                n -= 1;
            }

            d -= 1;
        }

        let three_sevenths = Fraction::new(3, 7);
        let found_fraction = match discovered_fractions.binary_search(&three_sevenths) {
            Ok(index_of_three_sevenths) => Some(discovered_fractions[index_of_three_sevenths - 1]),
            Err(_) => None
        };

        match found_fraction {
            Some(frac) => format!("{}", frac),
            None => String::from("No fraciton found")
        }
    }
}
