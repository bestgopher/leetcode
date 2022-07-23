#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut r = 0;

        for &i in nums.iter() {
            r ^= i;
        }
        r
    }
}
