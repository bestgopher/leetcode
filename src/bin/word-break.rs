#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// dp[i]表示s[0..i]是否满足
    /// dp[j] 是否满足取决于dp[0] —> dp[j-n] 和 s[j-n..j]是否满足
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut n = s.len();

        let hash = word_dict
            .iter()
            .map(|x| &x[..])
            .collect::<std::collections::HashSet<&str>>();
        let mut dp = vec![false; n];
        dp[0] = hash.contains(&s[0..1]);

        for i in 1..n {
            if (dp[i - 1] && hash.contains(&s[i..i + 1])) || hash.contains(&s[0..i + 1]) {
                dp[i] = true;
                continue;
            }

            for j in 0..i {
                dp[i] = dp[j] && hash.contains(&s[j + 1..i + 1]);
                if dp[i] {
                    break;
                }
            }
        }

        dp[dp.len() - 1]
    }
}
