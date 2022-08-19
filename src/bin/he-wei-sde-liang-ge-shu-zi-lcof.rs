#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, nums.len() - 1);

        while i < j {
            if target - nums[i] > nums[j] {
                j -= 1;
            } else if target - nums[i] < nums[j] {
                i += 1;
            } else {
                break;
            }
        }

        vec![nums[i], nums[j]]
    }
}
