use std::cmp::min;
use super::Problem;

pub struct PathSumTwoWaysProblem {}

impl Problem for PathSumTwoWaysProblem {
    fn solve(&self) -> String {
        const GRID_DIM: usize = 80;

        let mut grid = vec![];
        let mut distances = [[0; GRID_DIM]; GRID_DIM];

        let bytes = include_bytes!("../data_files/0081_matrix.txt");
        let file_data = String::from_utf8_lossy(bytes);

        for line in file_data.lines() {
            grid.push(
                Vec::from_iter(line.trim().split(",").map(|s| s.parse::<u32>().unwrap()))
            );
        }

        // there's only one way to get to the top-left corner, so init that to the top-left of the grid
        distances[0][0] = grid[0][0];

        // The only way to get to the leftmost column is from the row above
        // So we init that to the distance of the elements above plus current grid element
        for i in 1..grid.len() {
            distances[i][0] = distances[i - 1][0] + grid[i][0];
        }

        // The only way to get to the top row is from the left
        // So we init that to the distance of the elements to the left plus current grid element
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