#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    // println!("{}", Solution::find_nth_digit(3));
    println!("{}", Solution::find_nth_digit(11));
    println!("{}", Solution::find_nth_digit(20));
    println!("{}", Solution::find_nth_digit(21));
    println!("{}", Solution::find_nth_digit(3322432));
    println!("{}", Solution::find_nth_digit(324365434));
    println!("{}", Solution::find_nth_digit(95493394));
    println!("{}", Solution::find_nth_digit(1000000000));
}

struct Solution;

impl Solution {
    pub fn find_nth_digit(mut n: i32) -> i32 {
        if n <= 9 {
            return n;
        }

        let mut n = n - 10;
        let mut i = 2;

        // 2^31为10位的数字
        while i < 9 {
            let s = (i as i32) * 9 * 10i32.pow(i - 1);
            if n >= s {
                n -= s;
            } else {
                break;
            }
            i += 1;
        }

        let num = n / i as i32 + 10i32.pow(i - 1);
        let digit = n % i as i32;

        num / (10i32.pow(i - digit as u32 - 1)) % 10
    }
}
