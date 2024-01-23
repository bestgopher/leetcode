#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(Solution::alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
}

struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut r = 0;
        let mut one = 1; // 1为1
        for i in 1..nums.len() {
            if nums[i] - nums[i - 1] == one {
                result += 1;
                one *= -1;
            } else {
                if nums[i] - nums[i - 1] == 1 {
                    result = 2;
                    one = -1;
                } else {
                    result = 1;
                    one = 1;
                }
            }

            r = r.max(result);
        }

        if r == 0 || r == 1 {
            -1
        } else {
            r
        }
    }
}
