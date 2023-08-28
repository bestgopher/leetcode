#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let (mut first, mut second) = (nums[0], i32::MAX);

        for &i in nums[1..].into_iter() {
            if i > second {
                return true;
            } else if i > first {
                second = i;
            } else {
                first = i;
            }
        }

        false
    }
}
