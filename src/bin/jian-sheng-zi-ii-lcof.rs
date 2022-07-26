#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!("{}", Solution::cutting_rope(120));
}

struct Solution;

impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let (a, b) = (n / 3, n % 3);

        let r = match b {
            0 => Self::calc(a as u32, 1),
            1 => Self::calc(a as u32 - 1, 4),
            2 => Self::calc(a as u32, 2),
            _ => unreachable!(),
        };

        (r % 1000000007) as i32
    }

    fn calc(x: u32, y: i64) -> i32 {
        let mut r = 1i64;
        for _i in 0..x {
            r *= 3;
            r %= 1000000007;
        }

        (r * y % 1000000007) as i32
    }
}
