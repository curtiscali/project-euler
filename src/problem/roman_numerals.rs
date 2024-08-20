use std::collections::HashMap;

use super::Problem;

fn from_roman(roman_numeral: &str) -> usize {
    let roman_numerals: HashMap<char, usize> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1_000)
    ]);

    let mut arabic: usize = 0;
    for digit in roman_numeral.chars() {
        arabic += roman_numerals[&digit];
    }

    return arabic;
}

pub struct RomanNumeralsProblem {}

impl Problem for RomanNumeralsProblem {
    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/0089_roman.txt");
        let file_data = String::from_utf8_lossy(bytes);

        let lines = file_data.lines();

        let mut total_characters = 0;

        for line in lines {
            total_characters += line.len();
        }

        return format!("{}", total_characters);
    }
}