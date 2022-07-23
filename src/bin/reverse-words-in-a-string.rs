#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let s = s.as_bytes();
        let (mut start, mut end) = (0, s.len() - 1);

        for i in 0..s.len() {
            if s[i] != b' ' {
                start = i;
                break;
            }
        }

        for i in (0..s.len()).rev() {
            if s[i] != b' ' {
                end = i;
                break;
            }
        }

        let mut s1 = vec![];

        for i in (start..end).rev() {
            if s[i] == b' ' && s[i + 1] != b' ' {
                s1.extend_from_slice(&s[i + 1..=end]);
                s1.push(b' ');
            } else if s[i] != b' ' && s[i + 1] == b' ' {
                end = i;
            }
        }

        s1.extend_from_slice(&s[start..=end]);
        String::from_utf8(Vec::from(s1)).unwrap()
    }
}
