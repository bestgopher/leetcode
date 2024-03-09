#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut n = &nums[..];
        let mut sum = None;
        while n.len() >= 2 {
            match sum {
                None => sum = Some(n[0] + n[1]),
                Some(x) if n[0] + n[1] != x => break,
                _ => {}
            }
            result += 1;
            n = &n[2..];
        }

        result
    }
}
