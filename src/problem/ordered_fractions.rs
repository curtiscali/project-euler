use super::Problem;
use crate::fractions::Fraction;

pub struct OrderedFractionsProblem {
    pub denom_limit: u32
}

impl Problem for OrderedFractionsProblem {
    fn solve(&self) -> String {
        let target_fraction = Fraction::new(3, 7);

        // Fundamentally this, problem involves neighboring fractions in a Farey sequence: https://en.wikipedia.org/wiki/Farey_sequence#Farey_neighbours
        // if two fractions, a/b & c/d are neighbors in a Farey sequence, then bc - ad = 1
        // we are given in the problem that c/d = 3/7 and the maximum possible denominator in this
        // sequence is 1,000,000.
        // if we set b = 1,000,000, we end up with the equation a = (bc - 1) / d
        // solving for that gives us our solution in O(1) time and memory
        let numerator = ((target_fraction.numerator * self.denom_limit) / target_fraction.denominator) - 1;
        let fraction = Fraction::new(numerator, self.denom_limit);

        format!("{}", fraction)
    }
}
