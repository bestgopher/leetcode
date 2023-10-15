#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut s = Vec::new();

        for i in 0..nums.len() {
            s.insert(index[i] as usize, nums[i]);
        }

        s
    }
}
