use super::Problem;

pub struct CombinatoricSelectionsProblem {}

impl Problem for CombinatoricSelectionsProblem {
    fn name(&self) -> String {
        String::from("Combinatoric Selections")
    }

    fn number(&self) -> u16 {
        53
    }

    fn solve(&self) -> String {
        let mut num_combos_over_limit = 0;
        let mut n = 100u128;
        let mut r = 0u128;
        let mut ncr = 1;

        while r < (n / 2) {
            let c_right = (ncr * (n - r)) / (r + 1);
            if c_right <= 1_000_000 {
                r += 1;
                ncr = c_right;
            } else {
                let c_upright = (ncr * (n - r)) / n;
                num_combos_over_limit += n - (2 * r) - 1;
                n -= 1;
                ncr = c_upright;
            }
        }


        return format!("{}", num_combos_over_limit);
    }
}
