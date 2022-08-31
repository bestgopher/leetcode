#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut r = vec![0; nums.len()];
        let max = (1 << maximum_bit) - 1;
        let len = r.len();
        let mut current = 0;
        for (i, v) in nums.into_iter().enumerate() {
            current ^= v;
            r[len - 1 - i] = (!current) & max;
        }

        r
    }
}
