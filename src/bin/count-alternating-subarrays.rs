#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut s = 1;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                s = 1;
            } else {
                s += 1;
            }
            result += s;
        }

        result
    }
}
