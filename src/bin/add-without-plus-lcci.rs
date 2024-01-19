#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn add(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let mut carry = (a & b) << 1 as u32;
            a ^= b;
            b = carry;
        }
        a
    }
}
