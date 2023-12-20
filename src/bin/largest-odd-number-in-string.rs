#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut s = num.as_bytes();

        for i in (0..s.len()).rev() {
            if (s[i] - b'0') % 2 == 1 {
                return String::from_utf8(s[..=i].to_vec()).unwrap();
            }
        }

        "".into()
    }
}
