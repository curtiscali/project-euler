use super::Problem;
use crate::strings::{normalized_list, word_value};

pub struct NamesScoresProblem {}

impl Problem for NamesScoresProblem {
    fn name(&self) -> String {
        String::from("Names Scores")
    }

    fn number(&self) -> u16 {
        22
    }

    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/names.txt");
        let file_data = String::from_utf8_lossy(bytes);

        let mut words = normalized_list(file_data.to_string());
        words.sort_by_key(|word| word.to_lowercase());

        let mut score_sum: u128 = 0;
        let mut i = 0;
        while i < words.len() {
            let score = (i + 1) as u128 * word_value(&words[i]) as u128;
            score_sum += score;

            i += 1; 
        }

        return format!("{}", score_sum);
    }
}
