use crate::fractions::farey_length;

use super::Problem;

pub struct CountingFractionsProblem {}

impl Problem for CountingFractionsProblem {
    fn solve(&self) -> String {
        const DENOM_LIMIT: u32 = 1_000_000;
        format!("{}", farey_length(DENOM_LIMIT, Some(false)))
    }
}
