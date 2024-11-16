use num::ToPrimitive;
use super::Problem;

fn fifth_power_sum(n: u128) -> u128 {
    let mut power_sum: u128 = 0;
    let mut number = n;
    let fifth_powers: [u128; 10] = [0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];

    while number > 0 {
        let digit = (number % 10).to_usize().unwrap();
        power_sum += fifth_powers[digit];
        number /= 10;
    }

    return power_sum;
}

pub struct DigitFifthPowersProblem {}

impl Problem for DigitFifthPowersProblem {
    fn name(&self) -> String {
        String::from("Digit Fifth Powers")
    }

    fn number(&self) -> u16 {
        30
    }

    fn solve(&self) -> String {
        let mut i = 1000;
        let mut matching_sum = 0;
        while i <= 1_000_000 {
            if i == fifth_power_sum(i) {
                matching_sum += i;
            }

            i += 1;
        }

        return format!("{}", matching_sum);
    }
}
