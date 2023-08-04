#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let mut dp = vec![vec![0; nums.len()]; nums.len()];

        for i in 0..nums.len() {
            dp[i][i] = nums[i];
        }

        if nums.len() > 1 {
            for i in (0..=nums.len() - 2).rev() {
                for j in i + 1..nums.len() {
                    dp[i][j] = (nums[i] - dp[i + 1][j]).max(nums[j] - dp[i][j - 1]);
                }
            }
        }

        dp[0][nums.len() - 1] >= 0
    }
}
