#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }

                if (nums[i] as i64 - nums[j] as i64).abs() <= t as i64
                    && (i as i32 - j as i32).abs() <= k
                {
                    return true;
                }
            }
        }

        false
    }
}
