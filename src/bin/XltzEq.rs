#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert!(Solution::is_palindrome(
        "A man, a plan, a canal: Panama".into()
    ));
    assert!(!Solution::is_palindrome("race a car".into()));
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
            if matches!(s[start], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9')
                && matches!(s[end], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9')
            {
                if s[start] == s[end] {
                    start += 1;
                    end -= 1;
                } else if matches!(s[start], b'a'..=b'z')
                    && matches!(s[end], b'A'..=b'Z')
                    && s[end] + 32 == s[start]
                {
                    start += 1;
                    end -= 1;
                } else if matches!(s[end], b'a'..=b'z')
                    && matches!(s[start], b'A'..=b'Z')
                    && s[start] + 32 == s[end]
                {
                    start += 1;
                    end -= 1;
                } else {
                    return false;
                }
            } else if !matches!(s[start], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') {
                start += 1;
            } else if !matches!(s[end], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') {
                end -= 1;
            } else {
                unreachable!()
            }
        }

        true
    }
}
