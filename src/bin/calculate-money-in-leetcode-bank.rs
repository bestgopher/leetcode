#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        const S: i32 = 49;

        let a = n / 7;
        let b = n % 7;
        ((S + a * 7) * a + (a * 2 + b + 1) * b) >> 1
    }
}
