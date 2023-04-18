#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ans = Vec::with_capacity(word1.len() + word2.len());
        let mut i = 0;

        while i < word1.len() && i < word2.len() {
            ans.push(word1.as_bytes()[i]);
            ans.push(word2.as_bytes()[i]);
            i += 1;
        }

        ans.extend(word1.as_bytes()[i..].iter());
        ans.extend(word2.as_bytes()[i..].iter());

        unsafe { String::from_utf8_unchecked(ans) }
    }
}
