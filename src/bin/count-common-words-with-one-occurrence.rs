#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut hash1 = std::collections::HashMap::new();
        let mut hash2 = std::collections::HashMap::new();

        for i in words1 {
            hash1.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        for i in words2 {
            hash2.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut result = 0;
        for (key, val) in hash1 {
            if val != 1 {
                continue;
            }

            if let Some(x) = hash2.get(&key) {
                if *x == 1 {
                    result += 1;
                }
            }
        }

        result
    }
}
