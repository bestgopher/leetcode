#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!("{}", Solution::my_pow(2.00000, 10));
    println!("{}", Solution::my_pow(2.10000, 3));
    println!("{}", Solution::my_pow(2.00000, -2));
}

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        Self::pow(x, n as i64)
    }

    fn pow(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let m = Self::pow(x, n.abs() / 2);
        let mut r = if n.abs() % 2 == 1 { m * m * x } else { m * m };

        if n < 0 {
            r = 1f64 / r;
        }

        r
    }
}
