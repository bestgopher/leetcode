#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(true, Solution::is_palindrome("A   aa".to_string()));
}

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let (mut start, mut end) = (0, s.len() - 1);
        let s = s.as_bytes();

        while start < end {
            if !s[start].is_ascii_alphanumeric() {
                start += 1;
                continue;
            }

            if !s[end].is_ascii_alphanumeric() {
                end -= 1;
                continue;
            }

            if s[start].to_ascii_lowercase() == s[end].to_ascii_lowercase() {
                start += 1;
                end -= 1;
            } else {
                return false;
            }
        }

        true
    }
}
