#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut a = x ^ y;
        let mut r = 0;

        while a > 0 {
            if a & 1 == 1 {
                r += 1;
            }

            a >>= 1;
        }

        r
    }

    /// s & s-1能去掉最后一个1
    /// 比如 s = 0b11001000
    /// 则 s - 1 = 0b11000111
    /// s & s1 = 0b11000000 去掉了第三个1
    pub fn hamming_distance1(x: i32, y: i32) -> i32 {
        let (mut s, mut r) = (x ^ y, 0);
        while s > 0 {
            s = s & (s - 1);
            r += 1;
        }

        r
    }
}
