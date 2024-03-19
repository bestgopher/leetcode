#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n % 2 == 1 {
            "a".repeat(n as usize)
        } else {
            let mut result = String::with_capacity(n as usize);
            result.push('a');
            result.extend((0..n - 1).map(|_x| 'b'));

            result
        }
    }
}
