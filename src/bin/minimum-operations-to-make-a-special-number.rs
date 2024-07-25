#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let mut result = num.len() as i32;
        let mut s = vec![false, false];
        let len = num.len() as i32 - 1;

        for (i, u) in num.bytes().enumerate().rev() {
            match u {
                b'0' => {
                    // 00结尾
                    if s[0] {
                        result = result.min(len - i as i32 + 1);
                    }
                    s[0] = true;
                }
                b'2' | b'7' => {
                    // 2, 5
                    // 7, 5
                    if s[1] {
                        result = result.min(len - i as i32 + 1);
                    }
                }
                b'5' => {
                    // 5，0
                    if s[0] {
                        result = result.min(len - i as i32 + 1);
                    }
                    s[1] = true;
                }
                _ => {}
            }
        }

        if s[0] {
            result = result.min(len);
        }
        result
    }
}
