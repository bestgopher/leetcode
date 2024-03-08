#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let n = n as i64;
        let k = target as i64;
        let m = n.min(k / 2);
        ((m * (m + 1) + (n - m - 1 + k * 2) * (n - m)) / 2 % 1_000_000_007) as i32
    }
}
