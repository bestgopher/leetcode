#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn smallest_string(s: String) -> String {
        let s = s.as_bytes();
        let mut s1 = vec![];
        let mut flag = false;
        for i in 0..s.len() {
            if s[i] == b'a' && i != s.len() - 1 {
                if flag {
                    s1.extend_from_slice(&s[i..]);
                    break;
                } else {
                    s1.push(b'a');
                }
            } else {
                flag = true;
                if s[i] != b'a' {
                    s1.push(s[i] - 1);
                } else {
                    s1.push(b'a');
                }
            }
        }

        unsafe { String::from_utf8_unchecked(s1) }
    }
}
