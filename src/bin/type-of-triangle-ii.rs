#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_unstable();

        if nums[0] + nums[1] <= nums[2] {
            return "none".into();
        }

        if nums[0] == nums[1] && nums[1] == nums[2] {
            "equilateral".into()
        } else if nums[0] == nums[1] || nums[1] == nums[2] {
            "isosceles".into()
        } else {
            "scalene".into()
        }
    }
}
