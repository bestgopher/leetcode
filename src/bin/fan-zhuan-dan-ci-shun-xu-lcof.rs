#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        if s.is_empty() {
            return Default::default();
        }

        // 先去掉首位的空格
        let mut s = s.trim().as_bytes();

        let mut i = s.len() - 1;
        let mut new = vec![];

        while !s.is_empty() {
            match s.last() {
                Some(&x) if x == b' ' => s = &s[..s.len() - 1],
                _ => {
                    let mut i = s.len() - 1;
                    while let Some(&x) = s.get(i) {
                        if x != b' ' {
                            if i == 0 {
                                break;
                            }
                            i -= 1;
                        } else {
                            break;
                        }
                    }

                    if i != 0 {
                        i += 1;
                    }

                    new.extend_from_slice(&s[i..]);
                    new.push(b' ');
                    s = &s[..i];
                }
            }
        }

        new.pop();

        String::from_utf8(new).unwrap()
    }
}
