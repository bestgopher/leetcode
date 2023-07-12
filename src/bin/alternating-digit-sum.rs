#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut r = 0;
        let mut flag = true;
        while n > 0 {
            if flag {
                r += n % 10;
            } else {
                r -= n % 10;
            }

            n /= 10;
            flag = !flag;
        }

        if !flag {
            r
        } else {
            -r
        }
    }
}
