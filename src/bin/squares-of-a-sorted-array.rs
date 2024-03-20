#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut r = vec![0; nums.len()];
        let mut k = nums.len() - 1;
        let mut nums = &nums[..];

        while !nums.is_empty() {
            if nums.len() == 1 {
                r[k] = nums[0].pow(2);
                nums = &nums[1..];
            } else {
                if nums[0].abs() > nums[nums.len() - 1].abs() {
                    r[k] = nums[0].pow(2);
                    nums = &nums[1..];
                } else {
                    r[k] = nums[nums.len() - 1].pow(2);
                    nums = &nums[..nums.len() - 1];
                }
            }

            k -= 1;
        }
        r
    }
}
