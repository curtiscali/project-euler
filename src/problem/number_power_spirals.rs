use super::Problem;

pub struct NumberPowerSpiralsProblem {
    pub spiral_size: u128
}

impl Problem for NumberPowerSpiralsProblem {
    fn solve(&self) -> String {
        let n = self.spiral_size / 2;

        // This formula is based on numerical analysis stemming from the formula for the sum at each level of the
        // spiral being 1 + sum((2n^2 + 1) + (2n^2 + 1) - 2n + (2n^2 + 1) - 4n + (2n^2 + 1) - 6n)
        // From there using the quadratic & linear sum formulas we can simplify & derive the formula below
        let spiral_sum = ((16 * n * n * n) + (30 * n * n) + (26 * n) + 3) / 3;

        return format!("{}", spiral_sum);
    }
}