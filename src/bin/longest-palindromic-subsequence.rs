#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut dp = vec![];
        for i in 0..s.len() {
            dp.push(vec![-1; s.len()]);
        }

        Self::dfs(s.as_bytes(), &mut dp, 0, s.len() - 1);

        dp[0][s.len() - 1]
    }

    pub fn dfs(s: &[u8], v: &mut [Vec<i32>], i: usize, j: usize) {
        if v[i][j] != -1 {
            return;
        }

        if i == j {
            v[i][j] = 1;
            return;
        }

        if i > j {
            v[i][j] = 0;
            return;
        }

        if j == 0 || i == s.len() - 1 {
            v[i][j] = 0;
            return;
        }

        if s[i] == s[j] {
            Self::dfs(s, v, i + 1, j - 1);
            v[i][j] = v[i + 1][j - 1] + 2;
        } else {
            Self::dfs(s, v, i + 1, j);
            Self::dfs(s, v, i, j - 1);

            v[i][j] = v[i + 1][j].max(v[i][j - 1]);
        }
    }
}
