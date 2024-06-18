#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let (mut x, mut y) = (1, 0);
        for i in s.bytes() {
            match i {
                b'A' => x = 2 * x + y,
                b'B' => y = 2 * y + x,
                _ => unreachable!(),
            }
        }

        x + y
    }
}
