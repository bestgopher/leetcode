#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let mut a = 0;
        let mut index = 0;
        loop {
            match (
                s1.as_bytes().get(index),
                s2.as_bytes().get(index),
                s3.as_bytes().get(index),
            ) {
                (Some(&x1), Some(&x2), Some(&x3)) if x1 == x2 && x2 == x3 => {
                    a += 1;
                    index += 1;
                }
                _ => break,
            }
        }

        if a == 0 {
            -1
        } else {
            (s1.len() - a + s2.len() - a + s3.len() - a) as _
        }
    }
}
