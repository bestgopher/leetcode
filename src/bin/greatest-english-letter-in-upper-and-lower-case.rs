#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cmp::max;

fn main() {}

struct Solution;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut r = "".to_owned();
        let mut v = vec![0u8; 26]; // 0代表没有，1代表小写，2代表大写

        for i in 0..s.as_bytes().len() {
            let m = s.as_bytes()[i];
            match m {
                b'A'..=b'Z' => {
                    if v[(m - b'A') as usize] == 1 {
                        r = r.max(s.as_str()[i..i + 1].to_string());
                        v[(m - b'A') as usize] = 3;
                    } else {
                        v[(m - b'A') as usize] = 2;
                    }
                }
                b'a'..=b'z' => {
                    if v[(m - b'a') as usize] == 2 {
                        r = r.max(s.as_str()[i..i + 1].to_string().to_uppercase());
                        v[(m - b'a') as usize] = 3;
                    } else {
                        v[(m - b'a') as usize] = 1;
                    }
                }
                _ => unreachable!(),
            }
        }

        r
    }
}
