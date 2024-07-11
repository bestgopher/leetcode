#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();

        for i in (1..nums.len()).step_by(2) {
            nums.swap(i, i - 1);
        }

        nums
    }
}
