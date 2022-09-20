#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::ops::Add;

fn main() {}

struct Solution;

impl Solution {
    pub fn is_fliped_string(s1: String, s2: String) -> bool {
        use std::ops::Add;

        if s1.len() != s2.len() {
            return false;
        }

        let mut new_s1 = s1.clone();
        new_s1.add(s1.as_str()).contains(&s2)
    }
}
