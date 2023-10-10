#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let mut nums = nums;
        let s = s.as_bytes();

        for i in 0..s.len() {
            if s[i] == b'R' {
                nums[i] += d;
            } else {
                nums[i] -= d;
            }
        }

        nums.sort();
        let mut result = 0i64;
        const MOD: i64 = 10i64.pow(9) + 7;
        let mut sum = 0i64;
        for i in 0..s.len() {
            result = (result + nums[i] as i64 * i as i64 - sum) % MOD;
            sum = (sum + nums[i] as i64) % MOD;
        }

        result as i32
    }
}
