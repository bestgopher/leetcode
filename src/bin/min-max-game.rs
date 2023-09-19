#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        while nums.len() != 1 {
            let mut new_nums = Vec::with_capacity(nums.len() / 2);
            for i in 0..nums.len() / 2 {
                if i % 2 == 0 {
                    new_nums.push(nums[2 * i].min(nums[2 * i + 1]));
                } else {
                    new_nums.push(nums[2 * i].max(nums[2 * i + 1]));
                }
            }

            nums = new_nums;
        }
        nums[0]
    }
}
