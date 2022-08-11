#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
}

struct Solution;

impl Solution {
    pub fn max_sub_array1(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        let mut result = dp[0];

        for i in 1..nums.len() {
            if dp[i - 1] > 0 {
                dp[i] = dp[i - 1] + nums[i];
            } else {
                dp[i] = nums[i];
            }

            if dp[i] > result {
                result = dp[i];
            }
        }

        result
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = nums[0];
        let mut result = dp;

        for &i in nums[1..].iter() {
            if dp > 0 {
                dp += i;
            } else {
                dp = i;
            }

            if dp > result {
                result = dp;
            }
        }

        result
    }
}
