#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn replace_spaces(s: String, length: i32) -> String {
        let mut s1 = String::new();
        for i in s.chars().take(length as usize) {
            if i == ' ' {
                s1.push_str("%20");
            } else {
                s1.push(i);
            }
        }

        s1
    }
}
