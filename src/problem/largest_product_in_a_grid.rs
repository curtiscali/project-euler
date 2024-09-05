use super::Problem;

pub struct LargestProductInAGridProblem {}

impl Problem for LargestProductInAGridProblem {
    fn solve(&self) -> String {
        let grid: [[u32; 20]; 2] = [
            [8, 2, 22, 97, 38, 15, 0, 40, 0, 75, 4, 5, 7, 78, 52, 12, 50, 77, 91, 8],
            [49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 0]
        ];

        let mut i = 0;
        while i < 16 {
            let mut j = 0;
            while j < 16 {
                j += 1;
            }

            i += 1;
        }

        return format!("");
    }
}
