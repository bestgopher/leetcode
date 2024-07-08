#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for &i in s.as_bytes() {
            match i {
                b'a' | b'b' => stack.push(i),
                b'c' => {
                    if stack.len() < 2 {
                        return false;
                    }

                    if stack.pop().unwrap() != b'b' || stack.pop().unwrap() != b'a' {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}
