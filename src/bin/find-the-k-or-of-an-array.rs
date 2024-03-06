#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        for mut i in 0..32 {
            if nums
                .iter()
                .map(|x| *x)
                .filter(|x| (x >> i) & 1 == 1)
                .count()
                >= k as usize
            {
                result += 2i32.pow(i as _);
            }
        }

        result
    }
}
