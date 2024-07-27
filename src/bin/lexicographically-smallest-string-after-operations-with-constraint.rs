#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        let mut r = 0;
        let mut k = k;
        let mut result = Vec::with_capacity(s.len());

        for i in s.bytes() {
            if k >= (i - b'a').min(b'z' + 1 - i) as i32 {
                result.push(b'a');
                k -= (i - b'a').min(b'z' + 1 - i) as i32;
            } else if k > 0 {
                result.push(i - k as u8);
                k = 0;
            } else {
                result.push(i);
            }
        }

        String::from_utf8(result).unwrap()
    }
}
