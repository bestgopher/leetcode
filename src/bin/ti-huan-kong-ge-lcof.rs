#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut result = String::new();
        for i in s.chars() {
            if i == ' ' {
                result.push_str("%20");
            } else {
                result.push(i);
            }
        }

        result
    }
}
