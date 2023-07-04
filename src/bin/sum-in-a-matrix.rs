#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        for i in nums.iter_mut() {
            i.sort_by(|a, b| a.cmp(b));
        }

        let mut result = 0;

        for i in 0..nums[0].len() {
            let mut max = 0;
            for j in 0..nums.len() {
                max = max.max(nums[j][i]);
            }

            result += max;
        }

        result
    }
}
