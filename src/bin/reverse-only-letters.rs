#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_only_letters(mut s: String) -> String {
        unsafe {
            let (mut start, mut end) = (0, s.len() - 1);
            let mut bytes = s.as_bytes_mut();

            while start < end {
                if !matches!(bytes[start], b'a'..=b'z'|b'A'..=b'Z') {
                    start += 1;
                } else if !matches!(bytes[end], b'a'..=b'z'|b'A'..=b'Z') {
                    end -= 1;
                } else {
                    bytes.swap(start, end);
                    start += 1;
                    end -= 1;
                }
            }
        }
        s
    }
}
