#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert!(Solution::check_inclusion("ab".into(), "eidbaooo".into()));
}

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut cnt = [0; 26];

        for i in 0..s1.len() {
            cnt[(s1.as_bytes()[i] - b'a') as usize] -= 1;
            cnt[(s2.as_bytes()[i] - b'a') as usize] += 1;
        }

        let mut diff = 0; // 统计不同字符的个数

        for &i in cnt.iter() {
            if i != 0 {
                diff += 1;
            }
        }

        if diff == 0 {
            return true;
        }

        for i in s1.len()..s2.len() {
            let (a, b) = (s2.as_bytes()[i - s1.len()], s2.as_bytes()[i]);
            if a == b {
                continue;
            }

            if cnt[(a - b'a') as usize] == 0 {
                diff += 1;
            }

            cnt[(a - b'a') as usize] -= 1;

            if cnt[(a - b'a') as usize] == 0 {
                diff -= 1;
            }

            if cnt[(b - b'a') as usize] == 0 {
                diff += 1;
            }

            cnt[(b - b'a') as usize] += 1;

            if cnt[(b - b'a') as usize] == 0 {
                diff -= 1;
            }

            if diff == 0 {
                return true;
            }
        }

        false
    }
}
