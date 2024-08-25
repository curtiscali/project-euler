use super::Problem;

pub struct CoinSumProblem {
    pub total_pence: usize,
    pub denominations: Vec<usize>
}

impl Problem for CoinSumProblem {
    fn solve(&self) -> String {
        let mut solutions: Vec<usize> = vec![0; self.total_pence + 1];
        solutions[0] = 1;

        let denominations = self.denominations.clone();
        for k in denominations {
            let mut i = 0;
            while i <= self.total_pence - k {
                solutions[i + k] = solutions[i + k] + solutions[i];
                i += 1;
            }
        }

        return format!("{}", solutions[self.total_pence]);
    }
}