#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 设值为x
    /// (1 + x) * (x - 1 + 1) / 2= (x + n) * (n - x + 1) / 2
    /// (1 + x) * x = (n + x) * (n - x + 1)
    /// x**2 + x = n ** 2 - nx + n + nx - x**2 + x
    /// 2 * x ** 2 = n ** 2 + n
    pub fn pivot_integer(n: i32) -> i32 {
        let x = n * n + n;
        if x % 2 == 1 {
            return -1;
        }

        for i in 1..=n {
            if i * i == x / 2 {
                return i;
            } else if i * i > x / 2 {
                return -1;
            }
        }

        -1
    }
}
