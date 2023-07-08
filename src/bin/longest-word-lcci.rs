#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        words.sort_by(|x, y| {
            if x.len() == y.len() {
                x.cmp(y)
            } else {
                y.len().cmp(&x.len())
            }
        });

        let mut set = words
            .iter()
            .map(|x| x.as_bytes())
            .collect::<std::collections::HashSet<&[u8]>>();

        for i in words.iter() {
            set.remove(i.as_bytes());
            if Self::check(&set, i.as_bytes()) {
                return i.into();
            }
            set.insert(i.as_bytes());
        }

        "".into()
    }

    fn check(set: &std::collections::HashSet<&[u8]>, s: &[u8]) -> bool {
        if s.is_empty() || set.contains(s) {
            return true;
        }

        for i in 0..s.len() {
            if set.contains(&s[..i]) && Self::check(set, &s[i..]) {
                return true;
            }
        }

        false
    }
}
