#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn least_minutes(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }

        let mut x = 1;
        let mut r = 0;
        for i in 1.. {
            x *= 2;
            r = i as i32;
            if x >= n {
                break;
            }
        }

        r + 1
    }
}
