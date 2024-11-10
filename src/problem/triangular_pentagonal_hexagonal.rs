use crate::number_theory::linear_sum;
use super::Problem;

fn is_pentagonal(n: usize) -> bool {
    let root = ((24 * n) + 1) as f64;
    let positive_zero = (root.sqrt() + 1.0) / 6.0;

    return positive_zero.floor() - positive_zero == 0.0;
}

pub struct TriangularPentagonalHexagonalNumberProblem {}

impl Problem for TriangularPentagonalHexagonalNumberProblem {
    fn solve(&self) -> String {
        let mut n = 287;
        let mut triangle_num = linear_sum(n);
        let mut found_triple_thread = false;

        while !found_triple_thread {
            let triangle = linear_sum(n);

            if is_pentagonal(triangle) {
                found_triple_thread = true; 
                triangle_num = triangle;
            }

            n += 2;
        }

        return format!("{}", triangle_num);
    }
}
