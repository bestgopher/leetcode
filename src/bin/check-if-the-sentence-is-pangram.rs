#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut v = [0u8; 26];
        for i in sentence.bytes() {
            v[(i - b'a') as usize] = 1;
        }

        v.iter().all(|&x| x == 1)
    }
}
