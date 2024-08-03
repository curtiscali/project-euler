use crate::Problem;

pub struct SumSquareDifference {
    pub count: u128
}

impl Problem for SumSquareDifference {
    fn solve(&self) -> String {
        // this is derived from squaring the formula n(n + 1)/2
        let square_of_sum: u128 = ((self.count * self.count) * ((self.count + 1) * (self.count + 1))) / 4;

        // sum of squares = n(n+1)(2n+1)/6
        let sum_of_squares: u128 = (self.count * (self.count + 1) * (2 * self.count + 1)) / 6;

        return format!("{}", square_of_sum - sum_of_squares);
    }
}