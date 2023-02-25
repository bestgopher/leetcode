#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!("{}", Solution::divide(15, 2));
    println!("{}", Solution::divide(-7, 3));
    println!("{}", Solution::divide(7, -3));
    println!("{}", Solution::divide(-2, 3));
    println!("{}", Solution::divide(0, 3));
}

struct Solution;

impl Solution {
    pub fn divide(a: i32, b: i32) -> i32 {
        let mut sign: i64 = if a.signum() == b.signum() { 1 } else { -1 };

        let mut a = if a > 0 { a as i64 } else { 0 - a as i64 };
        let mut b = if b > 0 { b as i64 } else { 0 - b as i64 };

        let mut ans = 0 as i64;
        let mut r = 1;
        let mut new_b = b;
        loop {
            if a < b {
                break;
            }

            if a < new_b + new_b {
                ans += r;
                a -= new_b;
                r = 1;
                new_b = b;
                continue;
            }

            new_b += new_b;
            r += r;
        }

        let ans = if sign == 1 { ans } else { 0 - ans };

        if ans > i32::MAX as i64 {
            i32::MAX
        } else {
            ans as i32
        }
    }
}
