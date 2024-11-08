use super::Problem;

pub struct ConsecutivePositiveDivisorsProblem {}

impl Problem for ConsecutivePositiveDivisorsProblem {
    fn solve(&self) -> String {
        const UPPER_BOUND: u32 = 10_000_000;

        let mut sigma0_lookup = vec![1u32; UPPER_BOUND as usize];
        let mut i = 2;
        while i < sigma0_lookup.len() {
            let mut j = i;
            while j < sigma0_lookup.len() {
                sigma0_lookup[j] += 1;
                j += i;
            }

            i += 1;
        }

        let mut num_consecutive = 0u32;
        let mut n = 2;
        while n < UPPER_BOUND {
            if sigma0_lookup[n as usize] == sigma0_lookup[(n - 1) as usize] { 
                num_consecutive += 1;
            }

            n += 1;
        }

        format!("{}", num_consecutive)
    }
}
