use super::Problem;
use crate::triangles;

pub struct MaximumPathSumTwoProblem {}

impl Problem for MaximumPathSumTwoProblem {
    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/0067_triangle.txt");
        let file_data = String::from_utf8_lossy(bytes);
        
        let mut triangle = triangles::from_string(&file_data);
        let n = triangle.len();

        format!("{}", triangles::max_path_sum(&mut triangle, n - 1))
    }
}
