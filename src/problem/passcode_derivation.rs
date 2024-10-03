use super::Problem;

pub struct PasscodeDerivationProblem {}

impl Problem for PasscodeDerivationProblem {
    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/0079_keylog.txt");
        let file_data = String::from_utf8_lossy(bytes);

        format!("")
    }
}
