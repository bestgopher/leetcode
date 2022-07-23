#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{}", Solution::count_numbers_with_unique_digits(2));
    println!("{}", Solution::count_numbers_with_unique_digits(8));
    println!("{}", Solution::count_numbers_with_unique_digits(7));
}

struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        if n == 1 {
            return 10;
        }

        let mut sum = 9;

        for i in 0..n - 1 {
            sum *= 9 - i;
        }

        sum + Self::count_numbers_with_unique_digits(n - 1)
    }
}
