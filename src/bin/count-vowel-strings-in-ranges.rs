#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut s = vec![0; words.len() + 1];

        for (i, v) in words.iter().enumerate() {
            if matches!(v.as_bytes()[0], b'a' | b'e' | b'i' | b'o' | b'u')
                && matches!(v.as_bytes()[v.len() - 1], b'a' | b'e' | b'i' | b'o' | b'u')
            {
                s[i + 1] = s[i] + 1;
            } else {
                s[i + 1] = s[i];
            }
        }

        let mut result = Vec::with_capacity(queries.len());
        for i in queries.into_iter() {
            result.push(s[i[1] as usize + 1] - s[i[0] as usize]);
        }

        result
    }
}
