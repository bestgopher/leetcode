#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut s = std::collections::HashMap::new();
        let mut r = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if let Some(x) = s.get_mut(&(nums[i] * nums[j])) {
                    r += (*x * 8);
                    *x += 1;
                    continue;
                }

                s.insert(nums[i] * nums[j], 1);
            }
        }

        r
    }
}
