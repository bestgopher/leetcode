#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".into()];
        }
        let mut v = std::collections::HashSet::new();
        for i in Self::generate_parenthesis(n - 1) {
            for j in 0..i.len() {
                let mut s = String::new();
                s.push_str(&i[..j]);
                s.push_str("()");
                s.push_str(&i[j..]);
                v.insert(s);
            }
        }

        v.into_iter().collect()
    }
}
