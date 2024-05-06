#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut result = 0;

        for i in gain {
            result = result.max(current + i);
            current = current + i;
        }

        result
    }
}
