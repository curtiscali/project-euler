use super::Problem;

static TENS: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];

static TO_NINETEEN: [&str; 20] = [
    "", "one", "two",  "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
    "seventeen", "eighteen", "nineteen"
];

fn tens_length(n: usize) -> usize {
    let ones_digit = n % 10;
    let tens_digit = n / 10;

    let ones = TO_NINETEEN[ones_digit];
    let tens = TENS[tens_digit - 2];

    return tens.len() + ones.len();
}

pub struct NumberLetterCountsProblem {}

impl Problem for NumberLetterCountsProblem {
    fn name(&self) -> String {
        String::from("Number Letter Counts")
    }

    fn number(&self) -> u16 {
        17
    }

    fn solve(&self) -> String {
        const LENGTH_OF_ONE_THOUSAND: usize = 11;
        const LENGTH_OF_HUNDRED: usize = 7;
        const LENGTH_OF_AND: usize = 3;

        let mut total_chars: usize = 0;
        let mut i = 1;
        while i <= 1000 {
            if i < 20 {
                total_chars += TO_NINETEEN[i].len();
            } else if i < 100 {
                total_chars += tens_length(i);
            } else if i < 1000 {
                let hundreds_digit = i / 100;
                total_chars += TO_NINETEEN[hundreds_digit].len() + LENGTH_OF_HUNDRED;

                let tens_part = i - (hundreds_digit * 100);
                if tens_part < 20 {
                    total_chars += TO_NINETEEN[tens_part].len();
                    if tens_part != 0 {
                        total_chars += LENGTH_OF_AND;
                    }
                } else {
                    total_chars += tens_length(tens_part) + LENGTH_OF_AND;
                }
            } else {
                total_chars += LENGTH_OF_ONE_THOUSAND;
                
            }

            i += 1;
        }

        return format!("{}", total_chars);
    }
}
