#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 {
                    result += 2;
                }

                if i == 0 {
                    result += grid[i][j];
                } else {
                    if grid[i - 1][j] < grid[i][j] {
                        result += grid[i][j] - grid[i - 1][j];
                    }
                }

                if i == grid.len() - 1 {
                    result += grid[i][j];
                } else {
                    if grid[i + 1][j] < grid[i][j] {
                        result += grid[i][j] - grid[i + 1][j]
                    }
                }

                if j == 0 {
                    result += grid[i][j];
                } else {
                    if grid[i][j - 1] < grid[i][j] {
                        result += grid[i][j] - grid[i][j - 1];
                    }
                }

                if j == grid[0].len() - 1 {
                    result += grid[i][j];
                } else {
                    if grid[i][j + 1] < grid[i][j] {
                        result += grid[i][j] - grid[i][j + 1];
                    }
                }
            }
        }

        result
    }
}
