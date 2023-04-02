#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        Self::f(&nums[..], target)
    }

    fn f(nums: &[i32], target: i32) -> i32 {
        if nums.len() == 0 {
            if target == 0 {
                return 1;
            } else {
                return 0;
            }
        }

        Self::f(&nums[1..], target - nums[0]) + Self::f(&nums[1..], target + nums[0])
    }
}
