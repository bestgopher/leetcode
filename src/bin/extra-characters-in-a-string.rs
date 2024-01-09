#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut set: std::collections::HashSet<_> = dictionary.iter().map(|x| x.as_str()).collect();
        let mut dp = vec![vec![-1; s.len()]; s.len()];

        let n = s.len();
        let mut dp = vec![0; n + 1];
        for i in 1..=n {
            dp[i] = dp[i - 1] + 1;
            for j in 1..=i {
                if set.contains(&s[j - 1..i]) {
                    dp[i] = dp[i].min(dp[j - 1]);
                }
            }
        }
        dp[n]
    }
}
