#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut n = num;
        let mut r = 0;
        while n > 0 {
            if num % (n % 10) == 0 {
                r += 1;
            }

            n /= 10;
        }

        r
    }
}
