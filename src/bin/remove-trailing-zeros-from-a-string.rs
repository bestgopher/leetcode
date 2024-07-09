#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut num = num;
        loop {
            match num.as_bytes().last() {
                Some(b'0') => {
                    let _ = num.pop();
                }
                _ => break,
            }
        }

        num
    }
}
