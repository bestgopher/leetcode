#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut result = 0;
        for i in 0..sequence.len() {
            result = result.max(Self::d(&sequence[i..], &word));
        }

        result
    }

    fn d(mut sequences: &str, word: &str) -> i32 {
        let mut result = 0;
        while sequences.len() >= word.len() && sequences.starts_with(word) {
            result += 1;
            sequences = &sequences[word.len()..];
        }

        result
    }
}
