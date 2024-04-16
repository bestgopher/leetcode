#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        (1..=a.min(b))
            .filter(|x| a % *x == 0 && b % *x == 0)
            .count() as i32
    }
}
