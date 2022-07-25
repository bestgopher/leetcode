#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::cutting_rope(10), 36);
}

struct Solution;

impl Solution {
    /// 记住：剪成长度为3的最好
    pub fn cutting_rope1(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let (a, b) = (n / 3, n % 3);
        match b {
            0 => 3i32.pow(a as u32),
            1 => 3i32.pow(a as u32 - 1) * 4,
            2 => 3i32.pow(a as u32) * 2,
            _ => unreachable!(),
        }
    }

    pub fn cutting_rope(n: i32) -> i32 {
        let mut dp = vec![0i32; n as usize + 1];
        dp[2] = 1;

        for i in 3..n + 1 {
            for j in 2..i {
                dp[i as usize] = dp[i as usize].max((j * (i - j)).max(j * dp[(i - j) as usize]));
            }
        }

        dp[n as usize]
    }
}
