#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut string = String::new();
        let mut s = &text[..];

        while !s.is_empty() {
            if s.starts_with("&quot;") {
                string.push('"');
                s = &s[6..]
            } else if s.starts_with("&apos") {
                string.push('\'');
                s = &s[6..]
            } else if s.starts_with("&amp;") {
                string.push('&');
                s = &s[5..]
            } else if s.starts_with("&gt;") {
                string.push('>');
                s = &s[4..]
            } else if s.starts_with("&lt;") {
                string.push('<');
                s = &s[4..]
            } else if s.starts_with("&frasl;") {
                string.push('/');
                s = &s[7..]
            } else {
                string.push_str(&s[0..1]);
                s = &s[1..];
            }
        }

        string
    }
}
