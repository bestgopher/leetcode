#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut map: std::collections::HashSet<i32> = nums.iter().map(|x| *x).collect();
        let mut result = 0;

        for i in 0..nums.len() {
            if map.contains(&(nums[i] + diff)) && map.contains(&(nums[i] + diff + diff)) {
                result += 1;
            }
        }

        result
    }
}
