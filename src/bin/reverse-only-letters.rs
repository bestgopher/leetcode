#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_only_letters(mut s: String) -> String {
        let mut bytes = unsafe { s.as_bytes_mut() };
        let (mut start, mut end) = (0, s.len() - 1);

        while start < end {
            if !matches(bytes[start], b'a'..=b'z', b'A'..=b'Z') {}
        }
        s
    }
}
