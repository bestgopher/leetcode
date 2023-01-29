#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::min_distance("horse".into(), "ros".into()), 3);
}

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();

        let (b1, b2) = (word1.as_bytes(), word2.as_bytes());

        let mut d = vec![vec![0; n + 1]; m + 1];

        for i in 1..=n {
            d[0][i] = d[0][i - 1] + 1;
        }

        for i in 1..=m {
            d[i][0] = d[i - 1][0] + 1;
        }

        for i in 1..=m {
            for j in 1..=n {
                let mut l = d[i - 1][j - 1];
                if b1[i - 1] == b2[j - 1] {
                    l -= 1;
                }

                d[i][j] = d[i][j - 1].min(l).min(d[i - 1][j]) + 1;
            }
        }

        d[m][n]
    }
}
