use crate::problem::Problem;

pub struct MultiplesProblem {
    pub limit: i32
}

impl Problem for MultiplesProblem {
    fn solve(&self) -> String {
        let mut sum: i32 = 0;

        let mut i = 1;
        while i < self.limit {
            if i % 3 == 0 || i % 5 == 0 {
                sum = sum + i;
            }

            i = i + 1;
        }

        return format!("{}", sum);
    }
}