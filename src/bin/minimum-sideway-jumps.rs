#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]), 2);
    assert_eq!(Solution::min_side_jumps(vec![0, 1, 1, 3, 3, 0]), 0);
    assert_eq!(Solution::min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
}

struct Solution;

impl Solution {
    /// dp
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let mut dp = [1, 0, 1i64];
        // 起点都没障碍物，所以起点为0

        for i in 1..obstacles.len() {
            let a = obstacles[i];
            // 说明当前位置三个跑道都没有障碍物，因此
            let mut new_dp = [0, 0, 0];
            if a == 0 {
                new_dp[0] = dp[0].min(1 + dp[1].min(dp[2]));
                new_dp[1] = dp[1].min(1 + dp[0].min(dp[2]));
                new_dp[2] = dp[2].min(1 + dp[0].min(dp[1]));
            } else if a == 1 {
                new_dp[0] = i32::MAX as i64;
                new_dp[1] = dp[1].min(1 + dp[2]);
                new_dp[2] = dp[2].min(1 + dp[1]);
            } else if a == 2 {
                new_dp[0] = dp[0].min(1 + dp[2]);
                new_dp[1] = i32::MAX as i64;
                new_dp[2] = dp[2].min(1 + dp[0]);
            } else {
                new_dp[0] = dp[0].min(1 + dp[1]);
                new_dp[1] = dp[1].min(1 + dp[0]);
                new_dp[2] = i32::MAX as i64;
            }

            dp = new_dp;
        }

        *dp[..].into_iter().min().unwrap() as i32
    }
}
