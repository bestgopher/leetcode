#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        let mut nums = nums;
        let (mut start, mut end) = (0, nums.len() - 1);

        while start < end {
            while start < end && nums[start] % 2 == 1 {
                start += 1;
            }
            while start < end && nums[end] % 2 == 0 {
                end -= 1;
            }

            nums.swap(start, end);
        }

        nums
    }
}
