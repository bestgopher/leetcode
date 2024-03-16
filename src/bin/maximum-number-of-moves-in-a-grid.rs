#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; grid.len()];

        for i in (0..grid[0].len() - 1).rev() {
            let mut new_dp = vec![0; grid.len()];
            for j in (0..grid.len()) {
                if grid[j][i] < grid[j][i + 1] {
                    new_dp[j] = dp[j] + 1;
                }

                if j != 0 && grid[j][i] < grid[j - 1][i + 1] {
                    new_dp[j] = new_dp[j].max(dp[j - 1] + 1)
                }

                if j != grid.len() - 1 && grid[j][i] < grid[j + 1][i + 1] {
                    new_dp[j] = new_dp[j].max(dp[j + 1] + 1)
                }
            }

            dp = new_dp;
        }

        dp.into_iter().max().unwrap()
    }
}
