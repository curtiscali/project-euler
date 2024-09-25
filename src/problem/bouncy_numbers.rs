use super::Problem;

fn is_increasing(number: u32) -> bool {
    let mut max_digit = number % 10;
    let mut n = number / 10;

    while n > 0 {
        let digit = n % 10;
        if digit > max_digit {
            return false;
        }

        max_digit = digit;
        n /= 10;
    }

    return true;
}

fn is_decreasing(number: u32) -> bool {
    let mut min_digit = number % 10;
    let mut n = number / 10;

    while n > 0 {
        let digit = n % 10;
        if digit < min_digit {
            return false;
        }

        min_digit = digit;
        n /= 10;
    }

    return true;
}

fn is_bouncy(n: u32) -> bool {
    return !is_increasing(n) && !is_decreasing(n);
}

pub struct BouncyNumbersProblem {
    pub target_bouncy_percentage: f64
}

impl Problem for BouncyNumbersProblem {
    fn solve(&self) -> String {
        let mut total_numbers_count = 21780u32;
        let mut bouncy_numbers_count = 19602u32; // 90% of 21780 is 19602

        while (bouncy_numbers_count as f64 / total_numbers_count as f64) < self.target_bouncy_percentage {
            if is_bouncy(total_numbers_count) {
                bouncy_numbers_count += 1;
            }

            total_numbers_count += 1;
        }

        format!("{}", total_numbers_count)
    }
}
