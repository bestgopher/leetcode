#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_prefix_score(mut nums: Vec<i32>) -> Vec<i64> {
        let mut max = 0i64;
        let mut sum = 0;
        let mut result = vec![0; nums.len()];
        for i in 0..nums.len() {
            max = max.max(nums[i] as i64);
            let s = max + nums[i] as i64;
            result[i] = sum + s;
            sum += s;
        }

        result
    }
}
