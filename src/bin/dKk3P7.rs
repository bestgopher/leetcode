#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut v = [0; 26];
        let mut f = false;
        for i in 0..s.len() {
            if !f && s.as_bytes()[i] != t.as_bytes()[i] {
                f = true;
            }

            v[(s.as_bytes()[i] - b'a') as usize] += 1;
            v[(t.as_bytes()[i] - b'a') as usize] -= 1;
        }

        if !f {
            return false;
        }

        for i in v {
            if i != 0 {
                return false;
            }
        }

        true
    }
}
