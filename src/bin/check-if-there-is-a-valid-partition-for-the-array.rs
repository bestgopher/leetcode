#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut dp = vec![false; nums.len() + 1];
        dp[0] = true;
        for (i, &v) in nums.iter().enumerate() {
            if (i > 0 && dp[i - 1] && v == nums[i - 1])
                || (i > 1 && dp[i - 2] && v == nums[i - 1] && v == nums[i - 2])
                || (i > 1 && dp[i - 2] && v == nums[i - 1] + 1 && v == nums[i - 2] + 2)
            {
                dp[i + 1] = true
            }
        }

        println!("{dp:?}");

        dp[nums.len()]
    }
}
