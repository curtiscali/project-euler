use std::cmp::min;

use super::Problem;

pub struct PathSumTwoWaysProblem {}

impl Problem for PathSumTwoWaysProblem {
    fn solve(&self) -> String {
        const GRID_DIM: usize = 80;

        let mut grid: Vec<Vec<u32>> = vec![];
        let mut distances = [[0; GRID_DIM]; GRID_DIM];

        let bytes = include_bytes!("../data_files/0081_matrix.txt");
        let file_data = String::from_utf8_lossy(bytes);

        for line in file_data.lines() {
            grid.push(
                Vec::from_iter(line.trim().split(",").map(|s| s.parse::<u32>().unwrap()))
            );
        }

        distances[0][0] = grid[0][0];

        for i in 1..grid.len() {
            distances[i][0] = distances[i - 1][0] + grid[i][0];
        }

        for j in 1..grid[0].len() {
            distances[0][j] = distances[0][j - 1] + grid[0][j];
        }

        for i in 1..grid.len() {
            for j in 1..grid[i].len() {
                distances[i][j] = min(distances[i - 1][j], distances[i][j - 1]) + grid[i][j];
            }
        }

        format!("{}", distances[GRID_DIM - 1][GRID_DIM - 1])
    }
}