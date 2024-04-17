#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut result = vec![];

        for i in (2..nums.len()).step_by(3) {
            if nums[i] - nums[i - 2] > k {
                return vec![];
            }

            result.push(vec![nums[i], nums[i - 1], nums[i - 2]]);
        }

        result
    }
}
