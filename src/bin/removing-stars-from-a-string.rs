#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = vec![];
        for &i in s.as_bytes() {
            if i == b'*' {
                stack.pop();
            } else {
                stack.push(i);
            }
        }

        String::from_utf8(stack).unwrap()
    }
}
