#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        (nums[0] * nums[1] * nums[nums.len() - 1])
            .max(nums[nums.len() - 1] * nums[nums.len() - 2] * nums[nums.len() - 3])
    }
}
