#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{}", Solution::push_dominoes(".L.R...LR..L..".to_string()));
}

struct Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut s = dominoes.into_bytes();

        let mut slow = 0;

        for i in 0..s.len() {
            if s[i] == b'.' && i != s.len() - 1 {
                continue;
            }

            let mut fast = i;
            if s[slow] != b'R' && s[fast] == b'L' {
                while fast > slow {
                    if s[slow] == b'.' {
                        s[slow] = b'L';
                    }
                    slow += 1;
                }
            } else if s[slow] == b'R' && s[fast] == b'L' {
                while fast > slow {
                    s[fast] = b'L';
                    s[slow] = b'R';
                    fast -= 1;
                    slow += 1;
                }
            } else if s[slow] == b'R' && (s[fast] == b'R' || fast == s.len() - 1) {
                while fast >= slow {
                    if s[slow] == b'.' {
                        s[slow] = b'R';
                    }

                    slow += 1;
                }
            } else if s[slow] == b'R' && s[fast] == b'R' {
            }

            slow = i;
        }

        unsafe { String::from_utf8_unchecked(s) }
    }
}
