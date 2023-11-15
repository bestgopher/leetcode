#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let max = nums.into_iter().max().unwrap();
        max * k + (k - 1) * k / 2
    }
}
