#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        use std::iter::FromIterator;
        let mut hash = std::collections::HashMap::new();
        let mut result = 0;

        for mut i in words {
            let reverse = String::from_iter(i.chars().into_iter().rev());
            let count = hash.remove(&reverse).unwrap_or(0);
            match count {
                x if x > 1 => {
                    result += 1;
                    hash.insert(reverse, count - 1);
                }

                1 => result += 1,
                0 => {
                    hash.entry(i).and_modify(|x| *x += 1).or_insert(1);
                }
                _ => unreachable!(),
            }
        }

        result
    }
}
