#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        if n > 1 {
            n - 1
        } else {
            1
        }
    }
}
