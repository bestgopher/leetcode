#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::io::Read;

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut b = 0;

        for &i in blocks.as_bytes().into_iter().take(k as usize) {
            if i == b'B' {
                b += 1;
            }
        }

        let mut result = k - b;
        let block = blocks.as_bytes();

        for i in 1..=blocks.len() - k as usize {
            if block[i - 1] == b'B' {
                b -= 1;
            }

            if block[i - 1 + k as usize] == b'B' {
                b += 1;
            }

            result = result.min(k - b);
        }

        result
    }
}
