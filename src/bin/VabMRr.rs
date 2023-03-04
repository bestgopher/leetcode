#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::find_anagrams("cbaebabacd".into(), "abc".into()),
        vec![0, 6]
    );
    assert_eq!(
        Solution::find_anagrams("abab".into(), "ab".into()),
        vec![0, 1, 2]
    );
}

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }
        let mut v = [0; 26];
        for i in 0..p.len() {
            v[(s.as_bytes()[i] - b'a') as usize] += 1;
            v[(p.as_bytes()[i] - b'a') as usize] -= 1;
        }

        let mut ans = vec![];
        let mut diff = 0;
        for &i in v.iter() {
            if i != 0 {
                diff += 1;
            }
        }

        if diff == 0 {
            ans.push(0);
        }

        for i in p.len()..s.len() {
            if v[(s.as_bytes()[i - p.len()] - b'a') as usize] == 0 {
                diff += 1;
            }

            v[(s.as_bytes()[i - p.len()] - b'a') as usize] -= 1;
            if v[(s.as_bytes()[i - p.len()] - b'a') as usize] == 0 {
                diff -= 1;
            }

            if v[(s.as_bytes()[i] - b'a') as usize] == 0 {
                diff += 1;
            }

            v[(s.as_bytes()[i] - b'a') as usize] += 1;
            if v[(s.as_bytes()[i] - b'a') as usize] == 0 {
                diff -= 1;
            }

            if diff == 0 {
                ans.push((i - p.len() + 1) as i32);
            }
        }

        ans
    }
}
