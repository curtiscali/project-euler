use std::cmp::max;

pub fn from_string(data: &str) -> Vec<Vec<i32>> {
    let mut triangle = vec![];
    for line in data.lines() {
        let numbers = Vec::from_iter(
            line.trim().split(" ").map(|s| s.parse::<i32>().unwrap())
        );

        triangle.push(numbers);
    }

    let max_len = triangle[triangle.len() - 1].len();
    for row in &mut triangle {
        let num_needed_zeroes = max_len - row.len();

        for _ in 1..=num_needed_zeroes {
            row.push(0);
        }
    }

    triangle
}

pub fn max_path_sum(triangle: &Vec<Vec<i32>>, i: usize, j: usize, row: usize, col: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    if j == col {
        return 0;
    }

    if i == row - 1 {
        return triangle[i][j];
    }

    if dp[i][j] != -1 {
        return dp[i][j];
    }

    dp[i][j] = triangle[i][j] + max(
        max_path_sum(triangle, i + 1, j, row, col, dp),
        max_path_sum(triangle, i + 1, j + 1, row, col, dp)
    );

    dp[i][j]
}
