use crate::fractions::farey_length;
use super::Problem;

pub struct CountingFractionsProblem {}

impl Problem for CountingFractionsProblem {
    fn name(&self) -> String {
        String::from("Counting Fractions")
    }

    fn number(&self) -> u16 {
        72
    }

    fn solve(&self) -> String {
        const DENOM_LIMIT: u32 = 1_000_000;
        format!("{}", farey_length(DENOM_LIMIT, Some(false)))
    }
}
