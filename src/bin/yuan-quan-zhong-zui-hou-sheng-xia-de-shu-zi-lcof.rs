#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 常规解法，使用vec作为列表
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        let mut v = 0;

        for i in 2..n + 1 {
            v = (v + m) % i;
        }

        v
    }
}
