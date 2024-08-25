use crate::arithmetic::linear_sum;

use super::Problem;

fn num_rectangles_in_grid(m: usize, n: usize) -> usize {
    return linear_sum(m) * linear_sum(n);
}

pub struct CountingRectanglesProblem {
    pub target_rectangle_count: usize
}

impl Problem for CountingRectanglesProblem {
    fn solve(&self) -> String {
        let upper_bound = (self.target_rectangle_count as f64 / 2.0).sqrt() as usize;
        let mut min_diff = self.target_rectangle_count;

        let mut x = 0;
        let mut y = 0;

        let mut m = 1;
        while m <= upper_bound {
            let mut n = m + 1;
            while n <= upper_bound {
                let rectangles_in_grid = num_rectangles_in_grid(m, n);

                if rectangles_in_grid > self.target_rectangle_count {
                    n += 1;
                    continue;
                }

                let diff = self.target_rectangle_count - rectangles_in_grid;

                if diff < min_diff {
                    min_diff = diff;
                    x = m;
                    y = n;
                }

                n += 1;
            }

            m += 1;
        }

        return format!("{} * {} = {}", x, y, x * y);
    }
}