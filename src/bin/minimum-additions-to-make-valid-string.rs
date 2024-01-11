#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut result = 0;
        let mut index = 0usize;
        let word = word.as_bytes();
        while index < word.len() {
            match word[index] {
                b'a' => match (word.get(index + 1), word.get(index + 2)) {
                    (Some(b'b'), Some(b'c')) => index += 3,
                    (Some(b'b'), _) => {
                        index += 2;
                        result += 1;
                    }
                    (Some(b'c'), _) => {
                        index += 2;
                        result += 1;
                    }
                    _ => {
                        index += 1;
                        result += 2;
                    }
                },
                b'b' => match word.get(index + 1) {
                    Some(b'c') => {
                        index += 2;
                        result += 1;
                    }

                    _ => {
                        index += 1;
                        result += 2;
                    }
                },
                b'c' => {
                    index += 1;
                    result += 2;
                }
                _ => unreachable!(),
            }
        }

        result
    }
}
