#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut r = 1;
        let mut s = 0;

        while n > 0 {
            r *= (n % 10);
            s += (n % 10);
            n /= 10;
        }

        r - s
    }
}
