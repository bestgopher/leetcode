#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::str::FromStr;

fn main() {
    assert!(!Solution::is_scramble("abcde".into(), "caebd".into()));
    assert!(Solution::is_scramble("great".into(), "rgeat".into()));
    assert!(Solution::is_scramble("abca".into(), "caba".into()));
    assert!(Solution::is_scramble("a".into(), "a".into()));
    assert!(!Solution::is_scramble("ab".into(), "xa".into()));
    assert!(Solution::is_scramble("aaab".into(), "baaa".into()));
    assert!(!Solution::is_scramble(
        "ccabcbabcbabbbbcbb".into(),
        "bbbbabccccbbbabcba".into()
    ));
    assert!(Solution::is_scramble(
        "abcdbdacbdac".into(),
        "bdacabcdbdac".into()
    ));
}

struct Solution;

impl Solution {
    /// 暴力递归
    pub fn force_is_scramble(s1: String, s2: String) -> bool {
        if s1.len() <= 1 && s2.len() <= 1 {
            return s1 == s2;
        }

        let (b1, b2) = (s1.as_str(), s2.as_str());
        for i in 1..s1.len() {
            if Solution::force_is_scramble(
                String::from_str(&b1[0..i]).unwrap(),
                String::from_str(&b2[0..i]).unwrap(),
            ) && Solution::force_is_scramble(
                String::from_str(&b1[i..b1.len()]).unwrap(),
                String::from_str(&b2[i..b2.len()]).unwrap(),
            ) {
                return true;
            }

            if Solution::force_is_scramble(
                String::from_str(&b1[i..b1.len()]).unwrap(),
                String::from_str(&b2[0..b1.len() - i]).unwrap(),
            ) && Solution::force_is_scramble(
                String::from_str(&b1[0..i]).unwrap(),
                String::from_str(&b2[b2.len() - i..b2.len()]).unwrap(),
            ) {
                return true;
            }
        }

        false
    }

    /// 递归记忆化搜索
    pub fn recu_is_scramble(s1: String, s2: String) -> bool {
        // v[b1的开始索引][b2的开始索引][长度]
        let mut v = vec![vec![vec![Option::<bool>::None; s1.len() + 1]; s1.len()]; s1.len()];

        let (b1, b2) = (s1.as_bytes(), s2.as_bytes());

        Self::check((0, b1.len()), (0, b1.len()), b1, b2, &mut v)
    }

    pub fn check(
        b1_index: (usize, usize),
        b2_index: (usize, usize),
        b1: &[u8],
        b2: &[u8],
        v: &mut Vec<Vec<Vec<Option<bool>>>>,
    ) -> bool {
        assert_eq!(b1_index.1 - b1_index.0, b2_index.1 - b2_index.0);

        if b1_index.1 - b1_index.0 == 1 {
            let r = b1[b1_index.0] == b2[b2_index.0];
            v[b1_index.0][b2_index.0][1] = Some(r);
        }

        if let Some(x) = v[b1_index.0][b2_index.0][b1_index.1 - b1_index.0] {
            return x;
        }
        for i in 1..b1_index.1 - b1_index.0 {
            if Self::check(
                (b1_index.0, b1_index.0 + i),
                (b2_index.0, b2_index.0 + i),
                b1,
                b2,
                v,
            ) && Self::check(
                (b1_index.0 + i, b1_index.1),
                (b2_index.0 + i, b2_index.1),
                b1,
                b2,
                v,
            ) {
                v[b1_index.0][b2_index.0][b1_index.1 - b1_index.0] = Some(true);
                return true;
            }

            if Self::check(
                (b1_index.0, b1_index.0 + i),
                (b2_index.1 - i, b2_index.1),
                b1,
                b2,
                v,
            ) && Self::check(
                (b1_index.0 + i, b1_index.1),
                (b2_index.0, b2_index.1 - i),
                b1,
                b2,
                v,
            ) {
                v[b1_index.0][b2_index.0][b1_index.1 - b1_index.0] = Some(true);
                return true;
            }
        }

        v[b1_index.0][b2_index.0][b1_index.1 - b1_index.0] = Some(false);
        false
    }

    /// dp
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        let (b1, b2) = (s1.as_bytes(), s2.as_bytes());

        let mut v = vec![vec![vec![false; s1.len() + 1]; s1.len()]; s1.len()];

        for i in 0..n {
            for j in 0..n {
                v[i][j][1] = b1[i] == b2[j];
            }
        }

        for len in 2..=n {
            for i in 0..n {
                if i + len > n {
                    break;
                }

                for j in 0..n {
                    if j + len > n {
                        break;
                    }

                    // 从1个字符开始枚举
                    for k in 1..len {
                        if (v[i][j][k] && v[i + k][j + k][len - k])
                            || (v[i][j + len - k][k] && v[i + k][j][len - k])
                        {
                            v[i][j][len] = true;
                            break;
                        }
                    }
                }
            }
        }

        v[0][0][n]
    }
}
