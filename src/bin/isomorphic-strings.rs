#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut h1 = std::collections::HashMap::<u8, u8>::new();
        let mut h2 = std::collections::HashMap::<u8, u8>::new();
        let (s, t) = (s.as_bytes(), t.as_bytes());
        for i in 0..s.len() {
            let (mut r1, mut r2) = (false, false);
            if let Some(x) = h1.get(&s[i]) {
                r1 = *x != t[i];
            } else {
                h1.insert(s[i], t[i]);
            }

            if let Some(x) = h2.get(&t[i]) {
                r2 = *x != s[i];
            } else {
                h2.insert(t[i], s[i]);
            }

            if r1 || r2 {
                return false;
            }
        }

        true
    }
}
