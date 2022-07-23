#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        num == (1..=num / 2).into_iter().filter(|&x| num % x == 0).sum()
    }
}
