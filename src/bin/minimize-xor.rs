#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn minimize_xor(mut num1: i32, num2: i32) -> i32 {
        let mut c1 = num1.count_ones();
        let mut c2 = num2.count_ones();
        while c2 < c1 {
            num1 &= num1 - 1;
            c2 += 1;
        }
        while c2 > c1 {
            num1 |= num1 + 1;
            c2 -= 1
        }
        num1
    }
}
