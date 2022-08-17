#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, nums.len());

        while start < end {
            let middle = (start + end) / 2;
            if nums[middle] <= middle as i32 {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        start as i32
    }
}
