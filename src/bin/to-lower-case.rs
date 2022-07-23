#![allow(dead_code, unused, unused_variables)]

use serde::de::Unexpected::Str;

fn main() {}

struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut new = Vec::with_capacity(s.len());
        s.bytes().into_iter().for_each(|mut x| {
            if x >= b'A' && x <= b'Z' {
                x += 32;
            }
            new.push(x);
        });

        String::from_utf8(new).unwrap()
    }
}
