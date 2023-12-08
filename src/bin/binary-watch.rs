#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::fmt::format;

fn main() {}

struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut result = vec![];
        for i in 0..12i8 {
            let c = i.count_ones();
            if c > turned_on as u32 {
                continue;
            }

            for j in 0..60i8 {
                if j.count_ones() == turned_on as u32 - c {
                    result.push(format!("{}:{:0>2}", i, j));
                }
            }
        }

        result
    }
}
