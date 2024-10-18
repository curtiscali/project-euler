use super::Problem;

// Returns a 0-based weekday based on Zeller's congruence
// https://en.wikipedia.org/wiki/Zeller%27s_congruence#Common_simplification
// 0 = Saturday
// 1 = Sunday
// 2 = Monday
// 3 = Tuesday
// 4 = Wednesday
// 5 = Thursday
// 6 = Friday
fn get_weekday(year: u32, month: u32, day: u32) -> u32 {
    let m = if month <= 2 {
        month + 12
    } else {
        month
    };

    let y = if month <= 2 {
        year - 1
    } else {
        year
    };

    (day + ((13 * (m + 1)) / 5) + y + (y / 4) - (y / 100) + (y / 400)) % 7
}

pub struct CountingSundaysProblem {}

impl Problem for CountingSundaysProblem {
    fn solve(&self) -> String {
        const STARTING_YEAR: u32 = 1901;
        const MONTHS_PER_YEAR: u32 = 12;
        const SUNDAY: u32 = 1;

        // 0 = Jan 1, 1901
        let mut elapsed_months = 0u32;
        let mut num_sundays_on_first_of_month = 0u32;

        while elapsed_months < MONTHS_PER_YEAR * 100 {
            let year = STARTING_YEAR + (elapsed_months / MONTHS_PER_YEAR);
            let month = (elapsed_months % MONTHS_PER_YEAR) + 1;

            if get_weekday(year, month, 1) == SUNDAY {
                num_sundays_on_first_of_month += 1;
            }

            elapsed_months += 1;
        }

        format!("{}", num_sundays_on_first_of_month)
    }
}
