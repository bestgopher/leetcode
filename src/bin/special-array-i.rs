#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut i = nums[0] % 2;
        for x in nums[1..].into_iter() {
            if x % 2 == i {
                return false;
            }

            i = x % 2;
        }

        true
    }
}
