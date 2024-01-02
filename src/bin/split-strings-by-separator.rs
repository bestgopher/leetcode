#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result = vec![];
        for i in words {
            result.extend(
                i.split(separator)
                    .filter(|x| !x.is_empty())
                    .map(|x| x.to_string()),
            );
        }

        result
    }
}
