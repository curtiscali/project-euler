use super::Problem;
use crate::triangles;

pub struct MaximumPathSumTwoProblem {}

impl Problem for MaximumPathSumTwoProblem {
    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/0067_triangle.txt");
        let file_data = String::from_utf8_lossy(bytes);
        
        let triangle = triangles::from_string(&file_data);
        let n = triangle.len();
        let mut dp: Vec<Vec<i32>> = Vec::with_capacity(n);

        for _ in 0..n {
            dp.push(vec![-1; n]);
        }

        format!("{}", triangles::max_path_sum(&triangle, 0, 0, n, n, &mut dp))
    }
}
