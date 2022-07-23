#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() {
            return;
        }
        let (mut s1, mut s2) = (0, s.len() - 1);

        while s1 < s2 {
            s.swap(s1, s2);
            s1 += 1;
            s2 -= 1;
        }
    }
}
