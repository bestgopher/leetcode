#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |x, y| x ^ y)
    }
}
