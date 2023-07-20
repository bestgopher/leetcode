#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    /// 取反，求最小和的连续子数组，则其余是最大和
    /// 两种情况：1.最大和子数组在数组中间
    ///         2.最大和子数组在数组两边，在数组两边就是最小和数组在中间
    pub fn max_subarray_sum_circular(mut nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut last = nums[0];

        let mut all_positive = last < 0; // 是否全是负数, 如果全是负数的话，就不能探究第二种情况
                                         // 第一种情况
        for i in 1..nums.len() {
            if last > 0 {
                last += nums[i];
            } else {
                last = nums[i];
            }
            all_positive = all_positive && nums[i] < 0;
            result = result.max(last);
        }

        if all_positive {
            return result;
        }

        // 第二种情况
        let mut sum: i32 = nums.iter().map(|x| *x).sum();
        let mut last = nums[0];
        for i in 1..nums.len() {
            if last > 0 {
                last = nums[i];
            } else {
                last += nums[i];
            }

            result = result.max(sum - last);
        }

        result
    }
}
