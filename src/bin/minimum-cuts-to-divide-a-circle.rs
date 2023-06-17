#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n % 2 == 0 {
            n / 2
        } else if n == 1 {
            0
        } else {
            n
        }
    }
}
