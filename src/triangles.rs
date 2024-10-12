pub fn from_string(data: &str) -> Vec<Vec<u32>> {
    let mut triangle = vec![];
    for line in data.lines() {
        let numbers = Vec::from_iter(
            line.trim().split(" ").map(|s| s.parse::<u32>().unwrap())
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

// This function makes use of a dynamic programming algo from https://www.geeksforgeeks.org/maximum-path-sum-triangle/
pub fn max_path_sum(triangle: &mut Vec<Vec<u32>>, m: usize) -> u32 {
    for i in (0..=m-1).rev() {
        for j in 0..=i {
            triangle[i][j] += if triangle[i + 1][j] > triangle[i + 1][j + 1] {
                triangle[i + 1][j]
            } else {
                triangle[i + 1][j + 1]
            };
        }
    }

    triangle[0][0]
}
