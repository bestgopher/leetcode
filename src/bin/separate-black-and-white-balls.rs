#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut index = 0;
        let mut result = 0;
        let s = s.as_bytes();
        for i in 0..s.len() {
            if s[i] == b'0' {
                result += (i - index) as i64;
                index += 1;
            }
        }

        result
    }
}
