#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut v = vec![];
        for &i in s.as_bytes() {
            if v.is_empty() {
                v.push(i);
            } else {
                let &m = v.last().unwrap();
                if (m > i && m - i == 32) || (m < i && i - m == 32) {
                    v.pop();
                } else {
                    v.push(i);
                }
            }
        }

        String::from_utf8(v).unwrap()
    }
}
