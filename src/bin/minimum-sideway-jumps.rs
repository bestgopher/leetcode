#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]), 2);
    assert_eq!(Solution::min_side_jumps(vec![0, 1, 1, 3, 3, 0]), 0);
    assert_eq!(Solution::min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
}

struct Solution;

impl Solution {
    /// dp 这个看不懂
    pub fn min_side_jumps1(obstacles: Vec<i32>) -> i32 {
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

    /// dp[i][j], 0 < i < 3, j < obstacles.len() 表示第i条跑道在第j点时的最少跳跃次数
    /// 所以dp[i][j] 可能有：1.如果有障碍物的话，就不符合要求
    ///                    2.没有障碍物，就是dp[i-1][j]或者是dp[i][n]+1的最小值(因为跳跃到[i,j]位置不值可以从[i-1,j]跳跃而来，而且还可以从j处的其他跑道而来。所以统一下跳跃的次数就行了。)
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        // 起点都没障碍物，所以起点为0
        let mut dp = [1, 0, 1i64];

        for i in 1..obstacles.len() {
            let a = obstacles[i]; // 障碍物
            let mut new_dp = [0, 0, 0];
            let mut min = i32::MAX as i64;

            for i in 0..3 {
                if a - 1 == i {
                    new_dp[i as usize] = i32::MAX as i64;
                } else {
                    new_dp[i as usize] = dp[i as usize];
                }

                min = min.min(new_dp[i as usize]);
            }

            // 因为任何点都可以从当前位置的其他点+1跳来，需要判断下是不是其他点+1跳过来的更短少

            for i in 0..3 {
                if a - 1 != i {
                    new_dp[i as usize] = new_dp[i as usize].min(min + 1);
                }
            }

            dp = new_dp;
        }

        *dp[..].into_iter().min().unwrap() as i32
    }
}
