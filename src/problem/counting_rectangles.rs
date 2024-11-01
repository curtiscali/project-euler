use crate::arithmetic::linear_sum;

use super::Problem;

fn num_rectangles_in_grid(m: u32, n: u32) -> u32 {
    return linear_sum(m) * linear_sum(n);
}

pub struct CountingRectanglesProblem {}

impl Problem for CountingRectanglesProblem {
    fn solve(&self) -> String {
        const TARGET_RECTANGLE_COUNT: u32 = 2_000_000;

        let mut min_diff = TARGET_RECTANGLE_COUNT;
        let mut x = 0;
        let mut y = 0;

        for m in 1..=100 {
            for n in (m + 1)..=100 {
                let rectangles_in_grid = num_rectangles_in_grid(m, n);

                if rectangles_in_grid > TARGET_RECTANGLE_COUNT {
                    continue;
                }

                let diff = TARGET_RECTANGLE_COUNT - rectangles_in_grid;

                if diff < min_diff {
                    min_diff = diff;
                    x = m;
                    y = n;
                }
            }
        }

        return format!("{} * {} = {}", x, y, x * y);
    }
}