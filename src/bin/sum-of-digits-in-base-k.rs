#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut sum = 0;
        let mut n = n;

        while n > 0 {
            sum += n % k;
            n = n / k;
        }

        sum
    }
}
