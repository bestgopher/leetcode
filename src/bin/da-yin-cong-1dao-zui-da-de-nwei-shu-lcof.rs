#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!("{:?}", Solution::print_numbers(3));
}

struct Solution;

impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        (1..=10i32.pow(n as u32) - 1).collect()
    }
}
