#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort();

        let mut left = -1;
        let mut right = 0;

        for i in (0..nums.len()) {
            if nums[i] == target {
                if left == -1 {
                    left = i as i32;
                }

                right = i as i32;
            }
        }
        if left != -1 {
            (left..=right).collect()
        } else {
            Default::default()
        }
    }
}
