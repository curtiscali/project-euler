use super::Problem;
use crate::number_theory::is_triangular;
use crate::strings::{normalized_list, word_value};

pub struct CodedTriangleNumbersProblem {}

impl Problem for CodedTriangleNumbersProblem {
    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/words.txt");
        let file_data = String::from_utf8_lossy(bytes);

        let words = normalized_list(file_data.to_string());

        let mut num_triangular_words = 0;
        for word in words {
            let word_value = word_value(&word);

            if is_triangular(word_value) {
                num_triangular_words += 1;
            }
        }

        return format!("{}", num_triangular_words);
    }
}
