#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut x = 0;
        let mut m = forts[0];
        for i in 1..forts.len() {
            if forts[i] == 0 {
                if m != 0 {
                    x += 1;
                }
            } else {
                if m != forts[i] {
                    r = r.max(x);
                }
                x = 0;
                m = forts[i];
            }
        }

        r
    }
}
