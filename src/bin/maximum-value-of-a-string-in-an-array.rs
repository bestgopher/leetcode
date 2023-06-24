#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter()
            .map(|s| s.parse().unwrap_or(s.len() as i32))
            .max()
            .unwrap()
    }
}
