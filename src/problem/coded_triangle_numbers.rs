use super::Problem;
use crate::arithmetic::is_triangular;

fn word_value(str: &str) -> usize {
    let mut word_value: usize = 0;
    for c in str.chars() {
        let alphabet_index = c as usize - 'A' as usize;
        word_value += alphabet_index + 1;
    }

    return word_value;
}

pub struct CodedTriangleNumbersProblem {}

impl Problem for CodedTriangleNumbersProblem {
    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/words.txt");
        let file_data = String::from_utf8_lossy(bytes);

        let mut num_triangular_words = 0;
        for word in file_data.split(",") {
            let quotes_removed = word.get(1..word.len() - 1).unwrap();
            let word_value = word_value(quotes_removed);

            if is_triangular(word_value) {
                num_triangular_words += 1;
            }
        }

        return format!("{}", num_triangular_words);
    }
}