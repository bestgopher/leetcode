#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut r = Vec::with_capacity(n.len() + 1);
        let mut n = n.as_bytes();
        let mut nega = false;
        if n[0] == b'-' {
            nega = true;
            n = &n[1..];
            r.push(b'-');
        }
        let x = x as u8 + b'0';
        let mut done = false;
        for &i in n {
            if done {
                r.push(i);
            } else {
                if nega {
                    if x < i {
                        r.push(x);
                        done = true;
                    }
                } else {
                    if x > i {
                        r.push(x);
                        done = true;
                    }
                }
                r.push(i);
            }
        }

        if !done {
            r.push(x);
        }

        unsafe { String::from_utf8_unchecked(r) }
    }
}
