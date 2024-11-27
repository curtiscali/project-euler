use std::collections::HashMap;

use super::Problem;
use crate::strings::normalized_list;

fn word_value(s: &String) -> u32 {
    let mut score = 0;
    let mut i = 0;
    for c in s.chars() {
        let d = c as u32 - 'A' as u32;
        score += d * 10u32.pow((s.len() - 1 - i) as u32);

        i += 1;
    }

    score
}

pub struct AnagramicSquaresProblem {}

impl Problem for AnagramicSquaresProblem {
    fn name(&self) -> String {
        String::from("Anagramic Squares")
    }

    fn number(&self) -> u16 {
        98
    }

    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/0081_matrix.txt");
        let file_data = String::from_utf8_lossy(bytes);

        let mut word_scores: HashMap<String, u32> = HashMap::new();

        for word in normalized_list(file_data.to_string()) {
            let word_value = word_value(&word);

            // TODO: 
            // Check if any anagrams of this word have been discovered
            // if not, add the word and its score to the map
            // Otherwise: find the anagram word & replace its value with the new one
        }

        format!("Not yet solved")
    }
}
