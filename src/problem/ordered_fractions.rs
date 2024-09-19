use std::collections::{BTreeSet, VecDeque};
use super::Problem;
use crate::fractions::Fraction;

struct SternBrocotParameters {
    left: Fraction,
    right: Fraction
}

pub struct OrderedFractionsProblem {
    pub denom_limit: u32
}

impl Problem for OrderedFractionsProblem {
    fn solve(&self) -> String {
        let source_fraction = Fraction::new(1, 3);
        let target_fraction = Fraction::new(1, 2);
        let mut generated_fractions: BTreeSet<Fraction> = BTreeSet::new();

        let mut stack: VecDeque<SternBrocotParameters> = VecDeque::with_capacity(self.denom_limit as usize);
        stack.push_front(SternBrocotParameters { left: source_fraction, right: target_fraction });

        while !stack.is_empty() {
            let SternBrocotParameters { left, right } = stack.pop_front().unwrap();

            let middle_fraction = Fraction::new(left.numerator + right.numerator, left.denominator + right.denominator);
            if middle_fraction.denominator <= self.denom_limit {
                generated_fractions.insert(middle_fraction);

                stack.push_front(SternBrocotParameters { left, right: middle_fraction });
                stack.push_front(SternBrocotParameters { left: middle_fraction, right });
            }
        }

        let sorted_fractions: Vec<Fraction> = generated_fractions.into_iter().collect();
        match sorted_fractions.binary_search(&Fraction::new(3, 7)) {
            Ok(index_of_three_sevenths) => format!("{}", sorted_fractions[index_of_three_sevenths - 1]),
            Err(_) => String::from("No fraction was found")
        }
    }
}
