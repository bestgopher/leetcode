#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // Self::calc(&nums, target)
        Self::calc(&nums, target)
    }

    pub fn calc(nums: &[i32], target: i32) -> i32 {
        if nums.is_empty() {
            if target == 0 {
                return 1;
            } else {
                return 0;
            }
        }

        if nums[0] == 0 {
            Self::calc(&nums[1..], target - nums[0]) * 2
        } else {
            Self::calc(&nums[1..], target - nums[0]) + Self::calc(&nums[1..], target + nums[0])
        }
    }
}
