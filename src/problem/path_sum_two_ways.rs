use std::{collections::BinaryHeap, i32};
use super::Problem;

const GRID_DIM: usize = 80;

#[derive(Eq, PartialEq)]
struct Cell {
    pub row: usize,
    pub col: usize,
    pub weight: i32
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn min_path_sum(dest: (usize, usize), grid: &Vec<Vec<i32>>) -> i32 {
    let (target_row, target_column) = dest;
    let mut visited = [[false; GRID_DIM]; GRID_DIM];

    let mut priority_queue = BinaryHeap::from([Cell { row: 0, col: 0, weight: grid[0][0] }]);
    while !priority_queue.is_empty() {
        let Cell { row, col, weight } = priority_queue.pop().unwrap();

        if visited[row][col] {
            continue;
        }

        visited[row][col] = true;

        if target_row == row && target_column == col {
            return weight;
        }

        if row + 1 < GRID_DIM {
            priority_queue.push(Cell { row: row + 1, col, weight: weight + grid[row + 1][col]});
        }

        if col + 1 < GRID_DIM {
            priority_queue.push(Cell { row, col: col + 1, weight: weight + grid[row][col + 1] });
        }
    }

    -1
}

pub struct PathSumTwoWaysProblem {}

impl Problem for PathSumTwoWaysProblem {
    fn solve(&self) -> String {
        let mut grid = vec![];

        let bytes = include_bytes!("../data_files/0081_matrix.txt");
        let file_data = String::from_utf8_lossy(bytes);

        for line in file_data.lines() {
            grid.push(
                Vec::from_iter(line.trim().split(",").map(|s| s.parse::<i32>().unwrap()))
            );
        }


        format!("{}", min_path_sum((GRID_DIM - 1, GRID_DIM - 1), &grid))
    }
}
