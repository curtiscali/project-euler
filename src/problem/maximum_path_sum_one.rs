use crate::triangles;
use super::Problem;

pub struct MaximumPathSumOneProblem {}

impl Problem for MaximumPathSumOneProblem {
    fn name(&self) -> String {
        String::from("Maximum Path Sum I")
    }

    fn number(&self) -> u16 {
        18
    }

    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/0018_triangle.txt");
        let file_data = String::from_utf8_lossy(bytes);
        
        let mut triangle = triangles::from_string(&file_data);
        let n = triangle.len();

        format!("{}", triangles::max_path_sum(&mut triangle, n - 1))
    }
}
