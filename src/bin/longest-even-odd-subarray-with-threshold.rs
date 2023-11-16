#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut result = 0;
        let mut index = 0;
        let mut left_index = nums.len();
        while index < nums.len() {
            if nums[index] > threshold {
                index += 1;
                left_index = nums.len();
                continue;
            }

            if left_index == nums.len() {
                if nums[index] % 2 == 0 {
                    left_index = index;
                    result = result.max(1);
                }
                index += 1;
            } else {
                if nums[index - 1] % 2 != nums[index] % 2 && nums[index] <= threshold {
                    result = result.max(index - left_index + 1);
                    index += 1;
                } else {
                    left_index = nums.len();
                }
            }
        }

        result as i32
    }
}
