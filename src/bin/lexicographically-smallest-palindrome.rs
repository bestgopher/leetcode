#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut s = s.into_bytes();
        let (mut i, mut j) = (0, s.len() - 1);

        while i < j {
            if s[i] > s[j] {
                s[i] = s[j];
            } else if s[i] < s[j] {
                s[j] = s[i];
            }

            i += 1;
            j -= 1;
        }

        unsafe { String::from_utf8_unchecked(s) }
    }
}
