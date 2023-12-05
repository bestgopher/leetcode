#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let (mut r1, mut r2) = (0, 0);
        let (mut s1, mut s2) = (b'1', b'0');

        for &i in s.as_bytes() {
            if i != s1 {
                r1 += 1;
            }

            if i != s2 {
                r2 += 1;
            }

            s1 = if s1 == b'1' { b'0' } else { b'1' };
            s2 = if s2 == b'1' { b'0' } else { b'1' };
        }

        r1.min(r2)
    }
}
