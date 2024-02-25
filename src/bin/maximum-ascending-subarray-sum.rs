#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut n = nums[0];
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                n += nums[i];
            } else {
                n = nums[i];
            }
            result = result.max(n);
        }

        result
    }
}
