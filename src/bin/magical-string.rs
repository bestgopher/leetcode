#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        let mut s = vec![1, 2, 2];
        let mut p = 2usize;

        while s.len() <= n as usize {
            if s[p] == 1 {
                if s[s.len() - 1] == 1 {
                    s.push(2)
                } else {
                    s.push(1)
                }
            } else {
                if s[s.len() - 1] == 1 {
                    s.push(2);
                    s.push(2);
                } else {
                    s.push(1);
                    s.push(1);
                }
            }
            p += 1;
        }

        let mut count = 0;

        for i in 0..n as usize {
            if s[i] == 1 {
                count += 1;
            }
        }
        count
    }
}
