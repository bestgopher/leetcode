#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        (2 * max_sum - (n - index).pow(2) + n * index) / (4 - 2 * index)
    }
}
