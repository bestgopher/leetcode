#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        words[left as usize..=right as usize]
            .into_iter()
            .filter(|x| x.starts_with(|x| matches!(x, 'a' | 'e' | 'i' | 'o' | 'u')))
            .filter(|x| x.ends_with(|x| matches!(x, 'a' | 'e' | 'i' | 'o' | 'u')))
            .count() as i32
    }
}
