#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_magic_index(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if nums[i] as usize == i {
                return nums[i];
            }
        }

        -1
    }
}
