#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;
        let (mut dpa, mut dpb, mut dpc) = (1, 1, 1);
        for i in 2..=n as usize {
            let next = (dp[dpa] * 2).min(dp[dpb] * 3).min(dp[dpc] * 5);
            dp[i] = next;

            if next == dp[dpa] * 2 {
                dpa += 1;
            }

            if next == dp[dpb] * 3 {
                dpb += 1;
            }

            if next == dp[dpc] * 5 {
                dpc += 1;
            }
        }

        dp[n as usize]
    }
}
