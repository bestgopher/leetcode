#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let (mut h1, mut h2) = ([0; 26], [0; 26]);
        for i in 0..word1.len() {
            h1[(word1.as_bytes()[i] - b'a') as usize] += 1;
            h2[(word2.as_bytes()[i] - b'a') as usize] += 1;
        }

        for i in 0..26 {
            match (h1[i], h2[i]) {
                (0, 0) => continue,
                (0, _) | (_, 0) => return false,
                _ => continue,
            }
        }

        h1.sort();
        h2.sort();

        for i in 0..h1.len() {
            if h1[i] != h2[i] {
                return false;
            }
        }

        true
    }
}
