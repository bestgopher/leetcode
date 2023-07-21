#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut a = [0, 1, 1];
        if n <= 2 {
            return a[n as usize];
        }

        for i in 3..=n {
            let old = a[0];
            a[0] = a[1];
            a[1] = a[2];
            a[2] = old + a[0] + a[1];
        }

        a[2]
    }
}
