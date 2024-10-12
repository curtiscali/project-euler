use super::Problem;

pub struct MaximumPathSumProblem {
    pub data_file: str
}

impl Problem for MaximumPathSumProblem {
    fn solve(&self) -> String {

        let bytes = include_str!(self.data_file);
        let file_data = String::from_utf8_lossy(bytes);
        format!("")
    }
}
