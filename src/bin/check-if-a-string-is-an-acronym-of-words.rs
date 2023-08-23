#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        if words.len() != s.len() {
            return false;
        }

        for i in 0..s.len() {
            if words[i].as_bytes()[0] != s.as_bytes()[i] {
                return false;
            }
        }

        true
    }
}
