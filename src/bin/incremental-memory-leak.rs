#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

impl Solution {
    pub fn mem_leak(mut memory1: i32, mut memory2: i32) -> Vec<i32> {
        let mut t = 1;

        loop {
            if t > memory1 && t > memory2 {
                return vec![t, memory1, memory2];
            }

            if memory2 > memory1 {
                memory2 -= t;
            } else {
                memory1 -= t;
            }

            t += 1;
        }
    }
}
