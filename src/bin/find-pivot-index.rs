#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut s = 0;

        for i in 0..nums.len() {
            if sum - nums[i] - s == s {
                return i as i32;
            }

            s += nums[i];
        }

        -1
    }
}
