#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut has_b = false;
        for &i in s.as_bytes() {
            if i == b'b' {
                has_b = true;
            } else if i == b'a' && has_b {
                return false;
            }
        }

        true
    }
}
