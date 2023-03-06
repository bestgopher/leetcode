#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::count_substrings("abc".into()), 3);
    assert_eq!(Solution::count_substrings("aaa".into()), 6);
}

struct Solution;

impl Solution {
    /// dp[i][j] = bool， 表示子串s[i..j] 是否为回文字符串
    ///             - dp[i+1][j-1] && s[i] == s[j], j - i > 1
    ///         -
    /// dp[i][j]
    ///         -
    ///             - s[i] == s[j], j-i==1
    ///
    pub fn count_substrings(s: String) -> i32 {
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut ans = 0;

        let mut len = 0;
        let s = s.as_bytes();

        while len <= s.len() {
            for i in 0..s.len() - len {
                if len <= 1 {
                    dp[i][i + len] = s[i] == s[i + len];
                } else {
                    dp[i][i + len] = s[i] == s[i + len] && dp[i + 1][i + len - 1];
                }

                if dp[i][i + len] {
                    ans += 1;
                }
            }

            len += 1;
        }

        ans
    }
}
