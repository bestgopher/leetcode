#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;

        for i in arr {
            if i % 2 == 0 {
                count = 0;
            } else {
                count += 1;
            }

            if count >= 3 {
                return true;
            }
        }

        false
    }
}
