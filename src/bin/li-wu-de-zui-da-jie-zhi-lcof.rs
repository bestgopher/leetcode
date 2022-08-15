#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
        Self::dp(&grid, &mut dp, (0, 0));

        dp[0][0]
    }

    fn dp(grid: &[Vec<i32>], dp: &mut [Vec<i32>], index: (usize, usize)) {
        if index.0 == grid.len() - 1 && index.1 == grid[0].len() - 1 {
            dp[index.0][index.1] = grid[index.0][index.1];
        }

        if index.0 + 1 < grid.len() && dp[index.0 + 1][index.1] == 0 {
            Self::dp(grid, dp, (index.0 + 1, index.1));
        }

        if index.1 + 1 < grid[0].len() && dp[index.0][index.1 + 1] == 0 {
            Self::dp(grid, dp, (index.0, index.1 + 1));
        }

        if index.0 + 1 < grid.len() && index.1 + 1 < grid[0].len() {
            dp[index.0][index.1] =
                grid[index.0][index.1] + dp[index.0 + 1][index.1].max(dp[index.0][index.1 + 1]);
        } else if index.0 + 1 < grid.len() {
            dp[index.0][index.1] = grid[index.0][index.1] + dp[index.0 + 1][index.1];
        } else if index.1 + 1 < grid[0].len() {
            dp[index.0][index.1] = grid[index.0][index.1] + dp[index.0][index.1 + 1];
        }
    }
}
