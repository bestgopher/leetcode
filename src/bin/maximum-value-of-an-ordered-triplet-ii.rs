#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cmp::max;

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max_diff = 0;
        let mut ans = 0;
        let mut pre_max = 0;
        for x in nums {
            ans = ans.max(x as i64 * max_diff as i64);
            max_diff = max_diff.max(pre_max - x);
            pre_max = pre_max.max(x);
        }

        ans
    }
}
