#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let mut result = nums[nums.len() - 1] as i64;

        for i in (0..nums.len() - 1).rev() {
            result = if nums[i] as i64 <= result {
                result + nums[i] as i64
            } else {
                nums[i] as i64
            };
        }

        result
    }
}
