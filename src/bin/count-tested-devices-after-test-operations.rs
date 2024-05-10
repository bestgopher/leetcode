#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in battery_percentages {
            if i > result {
                result += 1;
            }
        }

        result
    }
}
