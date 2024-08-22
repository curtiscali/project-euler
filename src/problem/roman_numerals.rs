use std::collections::HashMap;
use super::Problem;

fn from_roman(roman_number: &str, roman_numerals: &HashMap<char, usize>) -> usize {
    let mut arabic = 0;

    let mut i = 0;
    while i < roman_number.len() {
        let first = roman_number.chars().nth(i).unwrap();
        let second = roman_number.chars().nth(i + 1);

        let first_arabic_value = roman_numerals[&first];

        match second {
            Some(s) => {
                let second_arabic_value = roman_numerals[&s];

                if first_arabic_value >= second_arabic_value {
                    arabic += first_arabic_value;
                    i += 1;
                } else {
                    arabic += second_arabic_value - first_arabic_value;
                    i += 2;
                }
            },
            None => {
                arabic += first_arabic_value;
                i += 1;
            }
        }
    }

    return arabic;
}

fn to_roman(arabic: usize) -> String {
    let mut roman_numeral = String::new();
    let mut n = arabic;

    while n > 0 {
        if n >= 1000 {
            roman_numeral.push('M');
            n -= 1000;
        } else if n >= 900 {
            roman_numeral.push_str("CM");
            n -= 900;
        } else if n >= 500 {
            roman_numeral.push('D');
            n -= 500;
        } else if n >= 400 {
            roman_numeral.push_str("CD");
            n -= 400;
        } else if n >= 100 {
            roman_numeral.push('C');
            n -= 100;
        } else if n >= 90 {
            roman_numeral.push_str("XC");
            n -= 90;
        } else if n >= 50 {
            roman_numeral.push('L');
            n -= 50;
        } else if n >= 40 {
            roman_numeral.push_str("XL");
            n -= 40;
        } else if n >= 10 {
            roman_numeral.push('X');
            n -= 10;
        } else if n >= 9 {
            roman_numeral.push_str("IX");
            n -= 9;
        } else if n >= 5 {
            roman_numeral.push('V');
            n -= 5;
        } else if n >= 4 {
            roman_numeral.push_str("IV");
            n -= 4;
        } else if n >= 1 {
            roman_numeral.push('I');
            n -= 1;
        }
    }

    return roman_numeral;
}

pub struct RomanNumeralsProblem {}

impl Problem for RomanNumeralsProblem {
    fn solve(&self) -> String {
        let roman_numerals: HashMap<char, usize> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1_000)
        ]);

        let bytes = include_bytes!("../data_files/0089_roman.txt");
        let file_data = String::from_utf8_lossy(bytes);

        let lines = file_data.lines();

        let mut total_character_count = 0;
        let mut optimized_character_count = 0;

        for line in lines {
            total_character_count += line.len();

            let arabic = from_roman(line, &roman_numerals);
            let roman = to_roman(arabic);

            optimized_character_count += roman.len();
        }

        return format!("{}", total_character_count - optimized_character_count);
    }
}