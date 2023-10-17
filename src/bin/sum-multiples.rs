#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let f = |y: i32| -> i32 { (y + (n / y) * y) * (n / y) / 2 };
        f(3) + f(5) + f(7) - f(3 * 5) - f(3 * 7) - f(5 * 7) + f(3 * 5 * 7)
    }
}
