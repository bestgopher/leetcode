#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::count_substrings("abc".into()), 3);
    assert_eq!(Solution::count_substrings("aaa".into()), 6);
}

struct Solution;

impl Solution {
    /// 设f(i, j)表示是否s[i..j]为回文串(包含j)
    /// f(i, j) = f(i+1, j-1) && s[i] == s[j]
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut ans = 0;

        // 从长度为2开始遍历
        for sub_string_length in 0..s.len() {
            for j in 0..s.len() {
                if j + sub_string_length > s.len() - 1 {
                    break;
                }

                if sub_string_length == 0 {
                    dp[j][j + sub_string_length] = true;
                } else if sub_string_length == 1 {
                    dp[j][j + sub_string_length] = s[j] == s[j + sub_string_length];
                } else {
                    dp[j][j + sub_string_length] =
                        dp[j + 1][j + sub_string_length - 1] && s[j] == s[j + sub_string_length];
                }

                if dp[j][j + sub_string_length] {
                    ans += 1;
                }
            }
        }
        ans
    }
}
