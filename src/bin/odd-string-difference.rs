#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut d = std::collections::HashMap::new();

        for i in words {
            let mut s = vec![];
            for j in 1..i.as_bytes().len() {
                s.push(i.as_bytes()[j] - i.as_bytes()[j - 1]);
            }

            d.entry(s).or_insert(vec![]).push(i);
        }

        for (i, j) in d {
            if j.len() == 1 {
                return j[0].clone();
            }
        }

        unreachable!()
    }
}
