#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set = candy_type.iter().collect::<HashSet<_>>();
        set.len().min(candy_type.len() / 2) as _
    }
}
