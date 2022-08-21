#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::{io::Read, vec};

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let mut r = Vec::with_capacity(s.len());
        let bytes = s.as_bytes();
        for i in (n as usize..(n as usize + s.len())) {
            r.push(bytes[i % s.len()]);
        }

        String::from_utf8(r).unwrap()
    }
}
