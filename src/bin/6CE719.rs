#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut current = 0;

        for i in 1..temperature_a.len() {
            if temperature_a[i - 1].cmp(&temperature_a[i])
                == temperature_b[i - 1].cmp(&temperature_b[i])
            {
                current += 1;
            } else {
                current = 0;
            }

            result = result.max(current);
        }

        result
    }
}
