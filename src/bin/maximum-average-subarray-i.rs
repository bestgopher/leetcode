#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum = nums[0..k].iter().sum::<i32>();
        let mut argv = (sum as f64) / (k as f64);

        for i in k..nums.len() {
            sum = sum + nums[i] - nums[i - k];
            argv = argv.max((sum as f64) / (k as f64));
        }

        argv
    }
}
