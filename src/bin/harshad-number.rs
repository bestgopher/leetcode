#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut sum = 0;
        let mut x1 = x;

        while x1 > 0 {
            sum += x1 % 10;
            x1 /= 10;
        }

        if x % sum == 0 {
            sum
        } else {
            -1
        }
    }
}
