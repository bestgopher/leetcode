#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = vec![];
        for &i in s.as_bytes() {
            match i {
                b'B' | b'D' => match stack.last() {
                    Some(&x) if x + 1 == i => {
                        stack.pop();
                    }
                    _ => stack.push(i),
                },
                _ => stack.push(i),
            }
        }

        stack.len() as i32
    }
}
