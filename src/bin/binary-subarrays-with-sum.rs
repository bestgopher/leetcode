#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut map = std::collections::HashMap::<i32, i32>::new();
        let mut sum = 0;
        let mut result = 0;
        for i in nums {
            sum += i;
            map.entry(sum).and_modify(|x| *x += 1).or_insert(1);
            result += *map.get(&(sum - goal)).unwrap_or(&0);
        }

        result
    }
}
