#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// dp[i]表示前i个元素子序列的最大值
    /// 则 dp[i]的值可能为奇数或者偶数
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        // odd奇数，even 偶数
        let (mut odd, mut even, mut result) = (0, nums[0] as i64, nums[0] as i64);

        for &i in nums[1..].into_iter() {
            even = even.max(odd - i as i64);
            odd = odd.max(even + i as i64);
            result = odd.max(even);
        }
        result
    }
}
