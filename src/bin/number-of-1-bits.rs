#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut r = 0;
        let mut n = n;
        while n > 0 {
            if n % 2 == 1 {
                r += 1;
            }
            n /= 2;
        }

        r
    }
}
