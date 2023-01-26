#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::f32::consts::E;

fn main() {
    assert_eq!(
        Solution::longest_palindrome("aaaa".into()),
        "aaaa".to_string()
    );
}

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let length = s.len();
        if length == 1 {
            return s;
        }

        let bytes = s.as_bytes();

        let mut v: Vec<Vec<bool>> = (0..length)
            .map(|x| (0..length).map(|y| x == y).collect::<Vec<bool>>())
            .collect();

        let (mut start, mut end) = (0, 0);

        for i in 1..length {
            for start1 in 0..length {
                let end1 = start1 + i;
                if end1 >= length {
                    break;
                }

                if i == 1 {
                    v[start1][end1] = bytes[start1] == bytes[end1];
                } else {
                    v[start1][end1] = bytes[start1] == bytes[end1] && v[start1 + 1][end1 - 1];
                }

                if end1 - start1 >= end - start {
                    start = start1;
                    end = end1;
                }
            }
        }

        s[start..=end].to_owned()
    }
}
