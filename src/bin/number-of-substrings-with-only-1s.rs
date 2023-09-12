#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut n = 0i64;
        let mut r = 0i64;
        for &i in s.as_bytes() {
            match i {
                b'0' => {
                    r += (1 + n) * n / 2 % (1000000000 + 7);
                    n = 0;
                }
                b'1' => n += 1,
                _ => unreachable!(),
            }
        }

        r += (1 + n) * n / 2 % (1000000000 + 7);

        r as i32
    }
}
