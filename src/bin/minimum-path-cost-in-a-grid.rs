#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_path_cost(mut grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        // let mut v = vec![vec![0; grid[0].len()]; grid.len()];
        // for i in 0..grid[0].len() {
        //     v[grid.len() - 1][i] = grid[grid.len() - 1][i];
        // }

        for i in (0..grid.len() - 1).rev() {
            for j in 0..grid[0].len() {
                let mut min = move_cost[grid[i][j] as usize][0] + grid[i + 1][0] + grid[i][j];
                for k in 1..move_cost[grid[i][j] as usize].len() {
                    min = min.min(move_cost[grid[i][j] as usize][k] + grid[i + 1][k] + grid[i][j]);
                }

                grid[i][j] = min;
            }
        }

        *grid[0].iter().min().unwrap()
    }
}
