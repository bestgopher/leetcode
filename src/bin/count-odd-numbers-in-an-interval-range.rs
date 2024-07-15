#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut low = low;
        let mut high = high;
        if low % 2 == 0 {
            low += 1;
        }

        if high % 2 == 0 {
            high -= 1;
        }

        (high - low) / 2 + 1
    }
}
