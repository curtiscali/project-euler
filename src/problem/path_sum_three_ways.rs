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

fn min_path_sum(target_column: usize, grid: &Vec<Vec<i32>>) -> i32 {
    let mut visited = [[false; GRID_DIM]; GRID_DIM];

    let mut priority_queue = BinaryHeap::new();

    for i in 0..grid.len() {
        priority_queue.push(Cell { row: i, col: 0, weight: grid[i][0] });
    }

    while !priority_queue.is_empty() {
        let Cell { row, col, weight } = priority_queue.pop().unwrap();

        if visited[row][col] {
            continue;
        }

        visited[row][col] = true;

        if target_column == col {
            return weight;
        }

        if row + 1 < GRID_DIM {
            priority_queue.push(Cell { row: row + 1, col, weight: weight + grid[row + 1][col]});
        }

        if row >= 1 {
            priority_queue.push(Cell { row: row - 1, col, weight: weight + grid[row - 1][col] });
        }

        if col + 1 < GRID_DIM {
            priority_queue.push(Cell { row, col: col + 1, weight: weight + grid[row][col + 1] });
        }
    }

    -1
}

pub struct PathSumThreeWaysProblem {}

impl Problem for PathSumThreeWaysProblem {
    fn name(&self) -> String {
        String::from("Path Sum: Three Ways")
    }

    fn number(&self) -> u16 {
        82
    }

    fn solve(&self) -> String {
        const GRID_DIM: usize = 80;

        let mut grid = vec![];

        let bytes = include_bytes!("../data_files/0082_matrix.txt");
        let file_data = String::from_utf8_lossy(bytes);

        for line in file_data.lines() {
            grid.push(
                Vec::from_iter(line.trim().split(",").map(|s| s.parse::<i32>().unwrap()))
            );
        }

        format!("{}", min_path_sum(GRID_DIM - 1, &grid))
    }
}
