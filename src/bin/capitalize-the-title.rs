#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split(' ')
            .map(|x| {
                if x.len() <= 2 {
                    x.to_lowercase()
                } else {
                    let mut s = x.to_lowercase();
                    if s.as_bytes()[0] >= b'a' && s.as_bytes()[0] <= b'z' {
                        let w = s.as_bytes()[0] - b'a';
                        unsafe { s.as_bytes_mut()[0] = b'A' + w };
                    }

                    s
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
