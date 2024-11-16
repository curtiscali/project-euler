use super::Problem;

pub struct PowerfulDigitCountsProblem {}

impl Problem for PowerfulDigitCountsProblem {
    fn name(&self) -> String {
        String::from("Powerful Digit Counts")
    }

    fn number(&self) -> u16 {
        63
    }

    fn solve(&self) -> String {
        let mut num_powerful_numbers = 0u32;

        // Based on this extremely simple algorithm & solid reasoning
        // https://projecteuler.net/thread=63&page=8#430111
        for i in 1..10 {
            let n = 1.0 / (1.0 - (i as f64).log10());
            num_powerful_numbers += n.floor() as u32;
        }

        format!("{}", num_powerful_numbers)
    }
}
