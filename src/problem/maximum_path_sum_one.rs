use crate::triangles;
use super::Problem;

pub struct MaximumPathSumOneProblem {}

impl Problem for MaximumPathSumOneProblem {
    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/0016_triangle.txt");
        let file_data = String::from_utf8_lossy(bytes);
        
        let mut triangle = triangles::from_string(&file_data);
        let n = triangle.len();

        format!("{}", triangles::max_path_sum(&mut triangle, n - 1))
    }
}
