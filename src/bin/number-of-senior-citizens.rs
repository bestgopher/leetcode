#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter(|x| &x.as_str()[11..=12] > "60")
            .count() as i32
    }
}
